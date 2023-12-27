use sqlx::error::Error;
use async_trait::async_trait;


#[async_trait]
pub trait Repository<T> {
    async fn save(&mut self, payload: T) -> Result<(), Error>;
    async fn get_by_id(&self, id: i32) -> Result<Option<T>, Error>;
    async fn remove(&mut self, id: i32) -> Result<(), Error>;
    async fn patch(&mut self, id: i32, payload: T) -> Result<(), Error>;
}
