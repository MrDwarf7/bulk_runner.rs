#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error handler: {0}")]
    Generic(String),

    #[error("AutomateC error: {0}")]
    AutomateC(#[from] crate::internals::Error),

    #[error("Tokio error: {0}")]
    Tokio(#[from] tokio::task::JoinError),

    #[error("Semaphore error: {0}")]
    Semaphore(#[from] tokio::sync::AcquireError),

    #[error("Database error: {0}")]
    Database(#[from] deadpool_tiberius::SqlServerError),

    #[error("Deadpool failure during query run: {0}")]
    Deadpool(#[from] deadpool_tiberius::tiberius::error::Error),
}
