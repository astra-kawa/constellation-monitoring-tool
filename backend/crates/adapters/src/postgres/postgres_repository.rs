use async_trait::async_trait;
use domain::models::{Constellation, Satellite};
use ports::{errors::RepositoryError, outbound::ConstellationRepository};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

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
}
