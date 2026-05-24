use async_trait::async_trait;
use domain::models::{
    CartesianState, Constellation, EphemerisSource, KeplerianState, ReferenceFrame, Satellite,
    SatelliteEphemeris,
};
use ports::{errors::RepositoryError, outbound::ConstellationRepository};
use sqlx::{Pool, Postgres, QueryBuilder, postgres::PgPoolOptions};
use std::str::FromStr;

const SET_SATELLITE_QUERY: &str = "INSERT INTO constellation (id, data)
VALUES ($1, $2)
ON CONFLICT (id) DO UPDATE SET
    data = EXCLUDED.data
WHERE
    constellation.data IS DISTINCT FROM EXCLUDED.data";

const SET_EPHEMERIS_QUERY: &str = "INSERT INTO ephemeris (id, datetime, pos_x, pos_y, pos_z, vel_x, vel_y, vel_z, sma, ecc, inc, raan, arg, ta, reference_frame, source)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
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
    ephemeris.pos_x IS DISTINCT FROM EXCLUDED.pos_x,
    ephemeris.pos_y IS DISTINCT FROM EXCLUDED.pos_y,
    ephemeris.pos_z IS DISTINCT FROM EXCLUDED.pos_z,
    ephemeris.vel_x IS DISTINCT FROM EXCLUDED.vel_x,
    ephemeris.vel_y IS DISTINCT FROM EXCLUDED.vel_y,
    ephemeris.vel_z IS DISTINCT FROM EXCLUDED.vel_z,
    ephemeris.sma IS DISTINCT FROM EXCLUDED.sma,
    ephemeris.ecc IS DISTINCT FROM EXCLUDED.ecc,
    ephemeris.inc IS DISTINCT FROM EXCLUDED.inc,
    ephemeris.raan IS DISTINCT FROM EXCLUDED.raan,
    ephemeris.arg IS DISTINCT FROM EXCLUDED.arg,
    ephemeris.ta IS DISTINCT FROM EXCLUDED.ta,
    ephemeris.reference_frame IS DISTINCT FROM EXCLUDED.reference_frame,
    ephemeris.source IS DISTINCT FROM EXCLUDED.source";

pub struct PostgresRepository {
    pool: Pool<Postgres>,
}

impl PostgresRepository {
    pub async fn new(url: &str) -> Self {
        let pool: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(1)
            .connect(url)
            .await
            .expect("Bruh");

        Self { pool }
    }
}

#[async_trait]
impl ConstellationRepository for PostgresRepository {
    async fn set_constellation(&self, constellation: Constellation) -> Result<(), RepositoryError> {
        for satellite in constellation.satellites {
            let data_string =
                serde_json::to_string(&satellite.data).map_err(|_| RepositoryError::Other)?;

            sqlx::query(SET_SATELLITE_QUERY)
                .bind(satellite.id)
                .bind(data_string)
                .execute(&self.pool)
                .await
                .map_err(|_| RepositoryError::Other)?;
        }

        Ok(())
    }

    async fn get_constellation(&self) -> Result<Constellation, RepositoryError> {
        let satellite_records = sqlx::query!("SELECT * FROM constellation ORDER BY id ASC")
            .fetch_all(&self.pool)
            .await
            .map_err(|_| RepositoryError::Other)?;

        let mut satellites = Vec::<Satellite>::new();
        for record in satellite_records {
            satellites.push(Satellite {
                id: record.id,
                data: serde_json::from_str(&record.data.to_string())
                    .expect("JSON was not well-formatted"),
            });
        }

        Ok(Constellation { satellites })
    }

    async fn set_satellite(&self, satellite: Satellite) -> Result<(), RepositoryError> {
        sqlx::query(SET_SATELLITE_QUERY)
            .bind(satellite.id)
            .bind(sqlx::types::Json(satellite.data))
            .execute(&self.pool)
            .await
            .map_err(|err| RepositoryError::QueryError(err.to_string()))?;

        Ok(())
    }

    async fn get_satellite(&self, satellite_id: &str) -> Result<Satellite, RepositoryError> {
        let satellite_record =
            sqlx::query!("SELECT * FROM constellation WHERE id = $1", satellite_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|_| RepositoryError::Other)?;

        let satellite = Satellite {
            id: satellite_record.id,
            data: serde_json::from_str(&satellite_record.data.to_string())
                .expect("JSON was not well-formatted"),
        };

        Ok(satellite)
    }

    async fn delete_satellite(&self, satellite_id: &str) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM constellation WHERE id = $1", satellite_id)
            .execute(&self.pool)
            .await
            .map_err(|_| RepositoryError::Other)?;

        Ok(())
    }

    async fn set_constellation_ephemerides(
        &self,
        ephemerides: Vec<SatelliteEphemeris>,
    ) -> Result<(), RepositoryError> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(SET_EPHEMERIS_QUERY);

        // todo - investigate performance of this query
        query_builder.push_values(ephemerides, |mut binding, ephemeris| {
            for (cartesian_state, keplerian_state) in ephemeris
                .cartesian_ephemeris
                .iter()
                .zip(ephemeris.keplerian_ephemeris.iter())
            {
                binding
                    .push_bind(ephemeris.id.to_owned())
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
            }
        });

        let query = query_builder.build();
        query
            .execute(&self.pool)
            .await
            .map_err(|err| RepositoryError::QueryError(err.to_string()))?;

        Ok(())
    }

    async fn get_constellation_ephemerides(
        &self,
    ) -> Result<Vec<SatelliteEphemeris>, RepositoryError> {
        let constellation = self.get_constellation().await?;
        let mut ephemerides = Vec::<SatelliteEphemeris>::new();

        for satellite in constellation.satellites {
            let satellite_ephemeris_data = sqlx::query!(
                "SELECT * FROM ephemeris WHERE id = $1 ORDER BY datetime ASC",
                satellite.id
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|_| RepositoryError::Other)?;

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
                    reference_frame: ReferenceFrame::from_str(&record.reference_frame)
                        .map_err(|_| RepositoryError::Other)?,
                    source: EphemerisSource::from_str(&record.source)
                        .map_err(|_| RepositoryError::Other)?,
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
                    reference_frame: ReferenceFrame::from_str(&record.reference_frame)
                        .map_err(|_| RepositoryError::Other)?,
                    source: EphemerisSource::from_str(&record.source)
                        .map_err(|_| RepositoryError::Other)?,
                });
            }

            ephemerides.push(SatelliteEphemeris {
                id: satellite.id,
                cartesian_ephemeris,
                keplerian_ephemeris,
            });
        }

        Ok(ephemerides)
    }
}
