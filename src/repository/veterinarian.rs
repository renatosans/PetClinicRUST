use sqlx::error::Error;
use sqlx::postgres::PgPool;
use async_trait::async_trait;
use crate::repository::repository::Repository;
use crate::domain::veterinarian::Veterinarian;


pub struct VeterinarianRepository {
    pool: PgPool,
}

impl VeterinarianRepository {
    pub fn new(pool: PgPool) -> Self {
        VeterinarianRepository {
            pool: pool,
        }
    }
}

#[async_trait]
impl Repository<Veterinarian> for VeterinarianRepository {
    async fn save(&mut self, payload: Veterinarian) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }

    async fn get_by_id(&self, id: i32) -> Option<Veterinarian> {
        let veterinarian = Veterinarian::default();
        // TODO: implementar usando sqlx
        Some(veterinarian)
    }

    async fn remove(&mut self, id: i32) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }

    async fn patch(&mut self, id: i32, payload: Veterinarian) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }
}
