use async_trait::async_trait;
use domain::models::{
    CartesianState, Constellation, EphemerisSource, KeplerianState, ReferenceFrame, Satellite,
    SatelliteData, SatelliteEphemeris,
};
use ports::{errors::RepositoryError, outbound::ConstellationRepository};
use sqlx::{Pool, Postgres, QueryBuilder, postgres::PgPoolOptions};
use std::str::FromStr;
use tracing::{error, info, instrument};

const SET_SATELLITE_QUERY: &str = "INSERT INTO constellation (id, data)
VALUES ($1, $2)
ON CONFLICT (id) DO UPDATE SET
    data = EXCLUDED.data
WHERE
    constellation.data IS DISTINCT FROM EXCLUDED.data";

pub struct PostgresRepository {
    pool: Pool<Postgres>,
}

impl PostgresRepository {
    #[instrument(skip_all)]
    pub async fn new(url: &str) -> Result<Self, RepositoryError> {
        info!("Connecting to PostgreSQL DB");

        let pool: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(1)
            .connect(url)
            .await
            .map_err(|err| {
                let error = format!("DB connection error: {err}");
                error!(error);

                RepositoryError::Other(error)
            })?;

        info!("PostgresRepository initialized successfully");
        Ok(Self { pool })
    }

    #[instrument(skip_all)]
    async fn insert_ephemeris_batch<'a>(
        &self,
        rows: &[(
            &'a SatelliteEphemeris,
            &'a CartesianState,
            &'a KeplerianState,
        )],
    ) -> Result<(), RepositoryError> {
        if rows.is_empty() {
            return Ok(());
        }

        info!("Inserting ephemeris batch of size: {}", rows.len());

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            INSERT INTO ephemeris (
                id,
                datetime,
                pos_x,
                pos_y,
                pos_z,
                vel_x,
                vel_y,
                vel_z,
                sma,
                ecc,
                inc,
                raan,
                arg,
                ta,
                reference_frame,
                source
            )
            "#,
        );

        query_builder.push_values(
            rows,
            |mut binding, (ephemeris, cartesian_state, keplerian_state)| {
                binding
                    .push_bind(ephemeris.id.clone())
                    .push_bind(cartesian_state.datetime)
                    .push_bind(cartesian_state.pos_x)
                    .push_bind(cartesian_state.pos_y)
                    .push_bind(cartesian_state.pos_z)
                    .push_bind(cartesian_state.vel_x)
                    .push_bind(cartesian_state.vel_y)
                    .push_bind(cartesian_state.vel_z)
                    .push_bind(keplerian_state.sma_km)
                    .push_bind(keplerian_state.eccentricity)
                    .push_bind(keplerian_state.inclination_deg)
                    .push_bind(keplerian_state.raan_deg)
                    .push_bind(keplerian_state.arg_periapsis_deg)
                    .push_bind(keplerian_state.true_anomaly_deg)
                    .push_bind(cartesian_state.reference_frame.to_string())
                    .push_bind(cartesian_state.source.to_string());
            },
        );

        query_builder.push(
            r#"
            ON CONFLICT (id, datetime) DO UPDATE SET
                pos_x = EXCLUDED.pos_x,
                pos_y = EXCLUDED.pos_y,
                pos_z = EXCLUDED.pos_z,
                vel_x = EXCLUDED.vel_x,
                vel_y = EXCLUDED.vel_y,
                vel_z = EXCLUDED.vel_z,
                sma = EXCLUDED.sma,
                ecc = EXCLUDED.ecc,
                inc = EXCLUDED.inc,
                raan = EXCLUDED.raan,
                arg = EXCLUDED.arg,
                ta = EXCLUDED.ta,
                reference_frame = EXCLUDED.reference_frame,
                source = EXCLUDED.source
            WHERE
                ephemeris.pos_x IS DISTINCT FROM EXCLUDED.pos_x OR
                ephemeris.pos_y IS DISTINCT FROM EXCLUDED.pos_y OR
                ephemeris.pos_z IS DISTINCT FROM EXCLUDED.pos_z OR
                ephemeris.vel_x IS DISTINCT FROM EXCLUDED.vel_x OR
                ephemeris.vel_y IS DISTINCT FROM EXCLUDED.vel_y OR
                ephemeris.vel_z IS DISTINCT FROM EXCLUDED.vel_z OR
                ephemeris.sma IS DISTINCT FROM EXCLUDED.sma OR
                ephemeris.ecc IS DISTINCT FROM EXCLUDED.ecc OR
                ephemeris.inc IS DISTINCT FROM EXCLUDED.inc OR
                ephemeris.raan IS DISTINCT FROM EXCLUDED.raan OR
                ephemeris.arg IS DISTINCT FROM EXCLUDED.arg OR
                ephemeris.ta IS DISTINCT FROM EXCLUDED.ta OR
                ephemeris.reference_frame IS DISTINCT FROM EXCLUDED.reference_frame OR
                ephemeris.source IS DISTINCT FROM EXCLUDED.source
            "#,
        );

        let query = query_builder.build();

        query.execute(&self.pool).await.map_err(|err| {
            let error = format!("Failed to insert ephemeris batch: {}", err);
            error!(error);
            RepositoryError::QueryError(error)
        })?;

        info!("Ephemeris batch inserted successfully");

        Ok(())
    }
}

#[async_trait]
impl ConstellationRepository for PostgresRepository {
    #[instrument(skip_all)]
    async fn set_constellation(&self, constellation: Constellation) -> Result<(), RepositoryError> {
        info!("Setting constellation");

        for satellite in constellation.satellites {
            info!("Setting satellite: {}", satellite.id);

            let data_string = serde_json::to_string(&satellite.data).map_err(|err| {
                let error = format!("Failed to serialize satellite data: {}", err);
                error!("{}", error);

                RepositoryError::Other(error)
            })?;

            sqlx::query(SET_SATELLITE_QUERY)
                .bind(satellite.id)
                .bind(data_string)
                .execute(&self.pool)
                .await
                .map_err(|err| {
                    let error = format!("Failed to set satellite: {}", err);
                    error!("{}", error);

                    RepositoryError::QueryError(error)
                })?;
        }

        info!("Successfully set constellation");

        Ok(())
    }

    #[instrument(skip_all)]
    async fn get_constellation(&self) -> Result<Constellation, RepositoryError> {
        info!("Getting constellation");

        let satellite_records = sqlx::query!("SELECT * FROM constellation ORDER BY id ASC")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| {
                let error = format!("Failed to get constellation: {}", err);
                error!(error);

                RepositoryError::QueryError(error)
            })?;

        let mut satellites = Vec::<Satellite>::new();
        for record in satellite_records {
            let satellite_data: SatelliteData = serde_json::from_str(&record.data.to_string())
                .map_err(|err| {
                    let error = format!("Failed to parse satellite data: {}", err);
                    error!(error);

                    RepositoryError::Other(error)
                })?;

            satellites.push(Satellite {
                id: record.id,
                data: satellite_data,
            });
        }

        info!("Successfully got constellation");

        Ok(Constellation { satellites })
    }

    #[instrument(skip_all)]
    async fn set_satellite(&self, satellite: Satellite) -> Result<(), RepositoryError> {
        info!("Setting satellite: {}", &satellite.id);

        sqlx::query(SET_SATELLITE_QUERY)
            .bind(&satellite.id)
            .bind(sqlx::types::Json(satellite.data))
            .execute(&self.pool)
            .await
            .map_err(|err| {
                let error = format!("Failed to set satellite: {}", err);
                error!(error);

                RepositoryError::QueryError(error)
            })?;

        info!("Successfully set satellite: {}", &satellite.id);

        Ok(())
    }

    #[instrument(skip_all)]
    async fn get_satellite(&self, satellite_id: &str) -> Result<Satellite, RepositoryError> {
        info!("Getting satellite: {}", satellite_id);

        let satellite_record =
            sqlx::query!("SELECT * FROM constellation WHERE id = $1", satellite_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|err| {
                    let error = format!("Failed to get satellite: {}", err);
                    error!(error);

                    RepositoryError::QueryError(error)
                })?;

        let satellite_data: SatelliteData =
            serde_json::from_str(&satellite_record.data.to_string()).map_err(|err| {
                let error = format!("Failed to parse satellite data: {}", err);
                error!(error);

                RepositoryError::Other(error)
            })?;

        let satellite = Satellite {
            id: satellite_record.id,
            data: satellite_data,
        };

        info!("Successfully got satellite: {}", &satellite.id);

        Ok(satellite)
    }

    #[instrument(skip_all)]
    async fn delete_satellite(&self, satellite_id: &str) -> Result<(), RepositoryError> {
        info!("Deleting satellite: {}", satellite_id);

        sqlx::query!("DELETE FROM constellation WHERE id = $1", satellite_id)
            .execute(&self.pool)
            .await
            .map_err(|err| {
                let error = format!("Failed to delete satellite: {}", err);
                error!(error);

                RepositoryError::QueryError(error)
            })?;

        info!("Satellite deleted successfully: {}", satellite_id);

        Ok(())
    }

    #[instrument(skip_all)]
    async fn set_constellation_ephemerides(
        &self,
        ephemerides: &[SatelliteEphemeris],
    ) -> Result<(), RepositoryError> {
        info!("Setting constellation ephemerides");

        sqlx::query("TRUNCATE TABLE ephemeris")
            .execute(&self.pool)
            .await
            .map_err(|err| {
                let error = format!("Failed to truncate ephemeris table: {}", err);
                error!(error);
                RepositoryError::QueryError(error)
            })?;

        // todo - make batch size configurable
        let batch_size = 3000;
        let mut batch = Vec::with_capacity(batch_size);

        for ephemeris in ephemerides {
            info!("Inserting ephemeris for satellite: {}", ephemeris.id);

            for (cartesian_state, keplerian_state) in ephemeris
                .cartesian_ephemeris
                .iter()
                .zip(ephemeris.keplerian_ephemeris.iter())
            {
                batch.push((ephemeris, cartesian_state, keplerian_state));

                if batch.len() >= batch_size {
                    self.insert_ephemeris_batch(&batch).await?;
                    batch.clear();
                }
            }
        }

        if !batch.is_empty() {
            self.insert_ephemeris_batch(&batch).await?;
        }

        info!("Ephemerides inserted successfully");

        Ok(())
    }

    #[instrument(skip_all)]
    async fn get_constellation_ephemerides(
        &self,
    ) -> Result<Vec<SatelliteEphemeris>, RepositoryError> {
        info!("Fetching constellation ephemerides");

        let constellation = self.get_constellation().await?;
        let mut ephemerides = Vec::<SatelliteEphemeris>::new();

        for satellite in constellation.satellites {
            info!("Fetching ephemeris for satellite: {}", satellite.id);

            let satellite_ephemeris_data = sqlx::query!(
                "SELECT * FROM ephemeris WHERE id = $1 ORDER BY datetime ASC",
                satellite.id
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|err| {
                let error = format!(
                    "Failed to fetch ephemeris for satellite {}: {}",
                    satellite.id, err
                );
                error!(error);

                RepositoryError::QueryError(error)
            })?;

            let mut cartesian_ephemeris = Vec::<CartesianState>::new();
            let mut keplerian_ephemeris = Vec::<KeplerianState>::new();

            for record in satellite_ephemeris_data {
                cartesian_ephemeris.push(CartesianState {
                    datetime: record.datetime.and_utc(),
                    pos_x: record.pos_x,
                    pos_y: record.pos_y,
                    pos_z: record.pos_z,
                    vel_x: record.vel_x,
                    vel_y: record.vel_y,
                    vel_z: record.vel_z,
                    reference_frame: ReferenceFrame::from_str(&record.reference_frame).map_err(
                        |_| RepositoryError::Other("Invalid reference frame".to_string()),
                    )?,
                    source: EphemerisSource::from_str(&record.source).map_err(|_| {
                        RepositoryError::Other("Invalid ephemeris source".to_string())
                    })?,
                });

                // todo - assuming keplerian elements are not always defined for now
                keplerian_ephemeris.push(KeplerianState {
                    datetime: record.datetime.and_utc(),
                    sma_km: record.sma.unwrap_or(0.0),
                    eccentricity: record.ecc.unwrap_or(0.0),
                    inclination_deg: record.inc.unwrap_or(0.0),
                    raan_deg: record.raan.unwrap_or(0.0),
                    arg_periapsis_deg: record.arg.unwrap_or(0.0),
                    true_anomaly_deg: record.ta.unwrap_or(0.0),
                    reference_frame: ReferenceFrame::from_str(&record.reference_frame).map_err(
                        |_| RepositoryError::Other("Invalid reference frame".to_string()),
                    )?,
                    source: EphemerisSource::from_str(&record.source).map_err(|_| {
                        RepositoryError::Other("Invalid ephemeris source".to_string())
                    })?,
                });
            }

            ephemerides.push(SatelliteEphemeris {
                id: satellite.id,
                cartesian_ephemeris,
                keplerian_ephemeris,
            });
        }

        info!("Fetched constellation ephemerides successfully");

        Ok(ephemerides)
    }
}
