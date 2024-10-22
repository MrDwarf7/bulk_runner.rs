#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error handler: {0}")]
    Generic(String),

    // #[error("AutomateC error: {0}")]
    // AutomateC(#[from] crate::internals::Error),
    //
    #[error("Tokio error: {0}")]
    Tokio(#[from] tokio::task::JoinError),

    #[error("Semaphore error: {0}")]
    Semaphore(#[from] tokio::sync::AcquireError),

    #[error("Database error: {0}")]
    Database(#[from] deadpool_tiberius::SqlServerError),

    #[error("Deadpool failure during query run: {0}")]
    Deadpool(#[from] deadpool_tiberius::tiberius::error::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error parsing file: {0}")]
    Parse(#[from] std::string::FromUtf8Error),

    #[error("Bulk Runner Bots failure in query specific crate!: {0}")]
    BulkRunnerBots(#[from] bulk_runner_bots::Error),
}
