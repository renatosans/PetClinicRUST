use sqlx::error::Error;
use sqlx::postgres::PgPool;


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
