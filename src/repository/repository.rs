use sqlx::error::Error;
use async_trait::async_trait;


#[async_trait]
pub trait Repository<T> {
    async fn insert(&self, payload: T) -> Result<T, Error>;
    async fn get_by_id(&self, id: i32) -> Result<Option<T>, Error>;
    async fn delete(&self, id: i32) -> Result<u64, Error>;
    async fn patch(&self, id: i32, payload: T) -> Result<(), Error>;
}
