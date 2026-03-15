use async_trait::async_trait;
use domain::models::{Constellation, EciState, Satellite};
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

#[async_trait]
impl ConstellationRepository for PostgresRepository {
    async fn set_constellation(
        &self,
        _constellation: Constellation,
    ) -> Result<(), RepositoryError> {
        // for satellite in constellation.satellites {
        //     sqlx::query!(
        //         "INSERT INTO constellation VALUES (id, initial, pos_x, pos_y, pos_z, vel_x, vel_y, vel_z), ($1, $2, $3, $4, $5, $6, $7, $8)"
        //     );
        // }

        Ok(())
    }

    async fn get_constellation(&self) -> Result<Constellation, RepositoryError> {
        let satellite_records = sqlx::query!("SELECT * FROM constellation")
            .fetch_all(&self.pool)
            .await
            .map_err(|_| RepositoryError::Other)?;

        let mut satellites = Vec::<Satellite>::new();
        for record in satellite_records {
            satellites.push(Satellite {
                id: record.name,
                initial_state: EciState {
                    epoch: record.initial.unwrap(),
                    pos_x: record.pos_x.unwrap(),
                    pos_y: record.pos_y.unwrap(),
                    pos_z: record.pos_z.unwrap(),
                    vel_x: record.vel_x.unwrap(),
                    vel_y: record.vel_y.unwrap(),
                    vel_z: record.vel_z.unwrap(),
                },
            });
        }

        Ok(Constellation {
            satellites: Vec::<Satellite>::new(),
        })
    }
}
