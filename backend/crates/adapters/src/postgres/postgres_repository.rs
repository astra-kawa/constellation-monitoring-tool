use domain::models::{Constellation, Satellite};
use ports::{errors::RepositoryError, outbound::ConstellationRepository};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub struct PostgresRepository {
    pool: Pool<Postgres>,
    url: String,
}

impl PostgresRepository {
    pub async fn new(url: &str) -> Self {
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
        // for satellite in constellation.satellites {
        //     sqlx::query!(
        //         "INSERT INTO constellation VALUES (id, initial, pos_x, pos_y, pos_z, vel_x, vel_y, vel_z), ($1, $2, $3, $4, $5, $6, $7, $8)"
        //     );
        // }

        Ok(())
    }

    async fn get_constellation(&self) -> Result<Constellation, RepositoryError> {
        Ok(Constellation {
            satellites: Vec::<Satellite>::new(),
        })
    }
}
