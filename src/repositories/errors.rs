use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database connection error: {0}")]
    DatabaseConnectionError(anyhow::Error),
    #[error("Database error: {0}")]
    DatabaseError(anyhow::Error),
    #[error("Unknown error")]
    UnknownError,
}
