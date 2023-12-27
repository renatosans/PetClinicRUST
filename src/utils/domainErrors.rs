use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid data")]
    ValidationError,
    #[error("Item was not found")]
    NotFoundError,
    #[error("Item is already in the list")]
    DuplicationError,
}
