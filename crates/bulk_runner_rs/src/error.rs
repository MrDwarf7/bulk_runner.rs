#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error handler: {0}")]
    Generic(String),

    #[error("The AutomateC binary that is required to run this application couldn't be found!")]
    AutomateCNotFound,

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

    #[error("Bulk runner query error in binary: {0}")]
    BulkRunnerQuery(#[from] bulk_runner_query::Error),

    #[cfg(not(target_os = "windows"))]
    #[cfg(target_os = "linux")]
    #[error("One or more required database environment variables are not set")]
    DbEnvVarsNotSet,

    #[cfg(not(target_os = "windows"))]
    #[cfg(target_os = "linux")]
    #[error("The required 'PROD_SQL_USER' environment variable is not set")]
    DbEnvVarUserNotSet,

    #[cfg(not(target_os = "windows"))]
    #[cfg(target_os = "linux")]
    #[error("The required 'PROD_SQL_PASSWORD' environment variable is not set")]
    DbEnvVarPasswordNotSet,
}
