use bulk_runner_bots::{BulkRunnerBotsError, Result};
use deadpool_tiberius::tiberius::AuthMethod;

pub struct DbInfo {
    pub host: String,
    pub auth: AuthMethod,
    pub db:   String,
}

impl DbInfo {
    /// Retrieves the database information from environment variables.
    ///
    /// # Note
    /// These are checked on startup by the `Cli` structure when
    /// calling `new_with_env_check`.
    ///
    /// # Errors
    /// Will fail if any of the required environment variables are not set.
    ///
    pub fn auth_from_env() -> Result<Self> {
        #[cfg(windows)]
        #[cfg(not(unix))]
        {
            Self {
                host: crate::PROD_HOST.into(),
                auth: AuthMethod::Integrated,
                db:   crate::PROD_DB.into(),
            }
        }

        #[cfg(not(windows))]
        #[cfg(unix)]
        {
            let host = crate::PROD_HOST.into();
            let db = crate::PROD_DB.into();
            let auth = sql_auth_method_from_env()?;
            Ok(Self { host, auth, db })
        }
    }
}

impl TryFrom<String> for DbInfo {
    type Error = std::env::VarError;

    fn try_from(db_info: String) -> std::result::Result<Self, Self::Error> {
        let mut split = db_info.split_whitespace();
        let host = split.next().unwrap().to_string();
        let db = split.next().unwrap().to_string();

        #[cfg(windows)]
        #[cfg(not(unix))]
        {
            Self {
                host,
                auth: AuthMethod::Integrated,
                db,
            }
        }

        #[cfg(not(windows))]
        #[cfg(unix)]
        {
            let auth = sql_auth_method_from_env().unwrap_or_else(|e| {
                panic!("Failed to retrieve SQL authentication method from environment variables: {e:?}")
            });
            Ok(Self { host, auth, db })
        }
    }
}

/// Attempts to retrieve the SQL authentication method from environment variables.
///
/// # Environment Variables
/// * `PROD_SQL_USER` - The SQL username.
/// * `PROD_SQL_PASSWORD` - The SQL password.
///
/// # Errors
/// Will return an error if either environment variable is not set.
#[cfg(not(windows))]
#[cfg(unix)]
fn sql_auth_method_from_env() -> Result<AuthMethod> {
    let user = std::env::var("PROD_SQL_USER").map_err(BulkRunnerBotsError::EnvVar)?;
    let password = std::env::var("PROD_SQL_PASSWORD").map_err(BulkRunnerBotsError::EnvVar)?;
    Ok(deadpool_tiberius::tiberius::AuthMethod::sql_server(user, password))
}
