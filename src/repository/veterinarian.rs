use sqlx::error::Error;
use sqlx::postgres::PgPool;
use async_trait::async_trait;
use crate::repository::repository::Repository;
use crate::domain::veterinarian::Veterinarian;


pub struct VeterinarianRepository {
    pool: PgPool,
}

impl VeterinarianRepository {
    pub fn new(pool: &PgPool) -> Self {
        VeterinarianRepository {
            pool: pool.clone(),
        }
    }
}

#[async_trait]
impl Repository<Veterinarian> for VeterinarianRepository {
    async fn insert(&self, payload: Veterinarian) -> Result<Veterinarian, Error> {
        let inserted = sqlx::query_as!(Veterinarian, "INSERT INTO veterinarian (id, name, \"inscricaoCRMV\")
        VALUES ($1, $2, $3)
        RETURNING id, name, \"inscricaoCRMV\" as inscricao_crmv",
        payload.id,
        payload.name,
        payload.inscricao_crmv)
        .fetch_one(&self.pool)
        .await?;

        Ok(inserted)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Veterinarian>, Error> {
        match sqlx::query_as!(Veterinarian,"SELECT id, name, \"inscricaoCRMV\" as inscricao_crmv FROM veterinarian WHERE id = $1", id)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(veterinarian) => Ok(veterinarian),
            Err(err) => Err(Error::from(err))
        }
    }

    async fn delete(&self, id: i32) -> Result<u64, Error> {
        let query_result = sqlx::query!("DELETE FROM veterinarian WHERE id = $1", id)
        .execute(&self.pool)
        .await
        .map_err(Error::from)?;

        Ok(query_result.rows_affected())
    }

    async fn patch(&self, _id: i32, _payload: Veterinarian) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }
}
