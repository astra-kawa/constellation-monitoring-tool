use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::models::{Constellation, EciState, Satellite};
use ports::{errors::RepositoryError, outbound::ConstellationRepository};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

const SET_SATELLITE_QUERY: &str =
    "INSERT INTO constellation (name, initial, pos_x, pos_y, pos_z, vel_x, vel_y, vel_z)
VALUES ($1, $2::timestamp, $3, $4, $5, $6, $7, $8)
ON CONFLICT (name) DO UPDATE SET
    initial = EXCLUDED.initial,
    pos_x = EXCLUDED.pos_x,
    pos_y = EXCLUDED.pos_y,
    pos_z = EXCLUDED.pos_z,
    vel_x = EXCLUDED.vel_x,
    vel_y = EXCLUDED.vel_y,
    vel_z = EXCLUDED.vel_z
WHERE
    constellation.initial IS DISTINCT FROM EXCLUDED.initial OR
    constellation.pos_x IS DISTINCT FROM EXCLUDED.pos_x OR
    constellation.pos_y IS DISTINCT FROM EXCLUDED.pos_y OR
    constellation.pos_z IS DISTINCT FROM EXCLUDED.pos_z OR
    constellation.vel_x IS DISTINCT FROM EXCLUDED.vel_x OR
    constellation.vel_y IS DISTINCT FROM EXCLUDED.vel_y OR
    constellation.vel_z IS DISTINCT FROM EXCLUDED.vel_z";

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
            sqlx::query(SET_SATELLITE_QUERY)
                .bind(satellite.id)
                .bind(satellite.initial_state.epoch.naive_utc().to_string())
                .bind(satellite.initial_state.pos_x)
                .bind(satellite.initial_state.pos_y)
                .bind(satellite.initial_state.pos_z)
                .bind(satellite.initial_state.vel_x)
                .bind(satellite.initial_state.vel_y)
                .bind(satellite.initial_state.vel_z)
                .execute(&self.pool)
                .await
                .map_err(|_| RepositoryError::Other)?;
        }

        Ok(())
    }

    async fn get_constellation(&self) -> Result<Constellation, RepositoryError> {
        let satellite_records = sqlx::query!("SELECT * FROM constellation ORDER BY name ASC")
            .fetch_all(&self.pool)
            .await
            .map_err(|_| RepositoryError::Other)?;

        let mut satellites = Vec::<Satellite>::new();
        for record in satellite_records {
            satellites.push(Satellite {
                id: record.name,
                initial_state: EciState {
                    epoch: {
                        let epoch = record.initial.unwrap().assume_utc();
                        DateTime::<Utc>::from_timestamp(epoch.unix_timestamp(), epoch.nanosecond())
                            .expect("database timestamp should be valid")
                    },
                    pos_x: record.pos_x.unwrap(),
                    pos_y: record.pos_y.unwrap(),
                    pos_z: record.pos_z.unwrap(),
                    vel_x: record.vel_x.unwrap(),
                    vel_y: record.vel_y.unwrap(),
                    vel_z: record.vel_z.unwrap(),
                },
            });
        }

        Ok(Constellation { satellites })
    }

    async fn set_satellite(&self, satellite: Satellite) -> Result<(), RepositoryError> {
        sqlx::query(SET_SATELLITE_QUERY)
            .bind(satellite.id)
            .bind(satellite.initial_state.epoch.naive_utc().to_string())
            .bind(satellite.initial_state.pos_x)
            .bind(satellite.initial_state.pos_y)
            .bind(satellite.initial_state.pos_z)
            .bind(satellite.initial_state.vel_x)
            .bind(satellite.initial_state.vel_y)
            .bind(satellite.initial_state.vel_z)
            .execute(&self.pool)
            .await
            .map_err(|err| RepositoryError::QueryError(err.to_string()))?;

        Ok(())
    }

    async fn get_satellite(&self, satellite_id: &str) -> Result<Satellite, RepositoryError> {
        let satellite_record =
            sqlx::query!("SELECT * FROM constellation WHERE name = $1", satellite_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|_| RepositoryError::Other)?;

        // todo - replace with conversion function
        let satellite = Satellite {
            id: satellite_record.name,
            initial_state: EciState {
                epoch: {
                    let epoch = satellite_record.initial.unwrap().assume_utc();
                    DateTime::<Utc>::from_timestamp(epoch.unix_timestamp(), epoch.nanosecond())
                        .expect("database timestamp should be valid")
                },
                pos_x: satellite_record.pos_x.unwrap(),
                pos_y: satellite_record.pos_y.unwrap(),
                pos_z: satellite_record.pos_z.unwrap(),
                vel_x: satellite_record.vel_x.unwrap(),
                vel_y: satellite_record.vel_y.unwrap(),
                vel_z: satellite_record.vel_z.unwrap(),
            },
        };

        Ok(satellite)
    }

    async fn delete_satellite(&self, satellite_id: &str) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM constellation WHERE name = $1", satellite_id)
            .execute(&self.pool)
            .await
            .map_err(|_| RepositoryError::Other)?;

        Ok(())
    }
}
