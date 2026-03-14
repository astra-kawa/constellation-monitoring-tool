use domain::models::{Constellation, Satellite};
use ports::{errors::RepositoryError, outbound::ConstellationRepository};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub struct PostgresRepository {
    pool: Pool<Postgres>,
    url: String,
}

impl PostgresRepository {
    async fn new(url: &str) -> Self {
        let pool: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(1)
            .connect(url)
            .await
            .expect("Bruh");

        Self {
            pool,
            url: url.to_owned(),
        }
    }
}

impl ConstellationRepository for PostgresRepository {
    async fn set_constellation(&self, constellation: Constellation) -> Result<(), RepositoryError> {
        for satellite in constellation.satellites {
            sqlx::query!("INSERT INTO constellation VALUES (), ()")
        }

        Ok(())
    }

    async fn get_constellation(&self) -> Result<Constellation, RepositoryError> {
        Ok(Constellation {
            satellites: Vec::<Satellite>::new(),
        })
    }
}
