#[cfg(not(windows))]
#[cfg(unix)]
use std::env::VarError;

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
    #[cfg(not(windows))]
    #[cfg(unix)]
    pub fn auth_from_env() -> Result<Self, VarError> {
        let host = crate::PROD_HOST.into();
        let db = crate::PROD_DB.into();
        let auth = sql_auth_method_from_env()?;
        Ok(Self { host, auth, db })
    }
}

// HACK: This is not supposed to be permenant - more a temporary solution to see if we can
/// pull the db info from the given file
#[cfg(windows)]
#[cfg(not(unix))]
impl From<String> for DbInfo {
    fn from(db_info: String) -> Self {
        let mut split = db_info.split_whitespace();
        let host = split.next().unwrap().to_string();
        let db = split.next().unwrap().to_string();
        Self {
            host,
            auth: AuthMethod::Integrated,
            db,
        }
    }
}

#[cfg(windows)]
#[cfg(not(unix))]
impl Default for DbInfo {
    fn default() -> Self {
        Self {
            host: crate::PROD_HOST.into(),
            auth: AuthMethod::Integrated,
            db:   crate::PROD_DB.into(),
        }
    }
}

#[cfg(not(windows))]
#[cfg(unix)]
impl From<String> for DbInfo {
    fn from(db_info: String) -> Self {
        let mut split = db_info.split_whitespace();
        let host = split.next().unwrap().to_string();
        let db = split.next().unwrap().to_string();

        let auth = sql_auth_method_from_env().expect("Failed to get SQL auth from env");
        Self { host, auth, db }
    }
}

/// Default implementation that pulls from environment variables.
///
/// # Panics
/// Will panic if the environment variables are not set.
#[cfg(not(windows))]
#[cfg(unix)]
impl Default for DbInfo {
    fn default() -> Self {
        let tried = DbInfo::auth_from_env();

        match tried {
            Ok(db_info) => db_info,
            Err(e) => panic!("Failed to get DbInfo from env: {e}"),
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
pub fn sql_auth_method_from_env() -> Result<AuthMethod, VarError> {
    let user = sql_user_from_env()?;
    let password = sql_password_from_env()?;
    Ok(deadpool_tiberius::tiberius::AuthMethod::sql_server(user, password))
}

/// Attempts to retrieve the SQL username from environment variables.
///
/// # Environment Variables
/// * `PROD_SQL_USER` - The SQL username.
///
/// # Errors
/// Returns an error if the environment variable is not set.
#[cfg(not(windows))]
#[cfg(unix)]
pub fn sql_user_from_env() -> Result<String, VarError> {
    std::env::var("PROD_SQL_USER")
}

/// Attempts to retrieve the SQL password from environment variables.
///
/// # Environment Variables
/// * `PROD_SQL_PASSWORD` - The SQL password.
///
/// # Errors
/// Returns an error if the environment variable is not set.
#[cfg(not(windows))]
#[cfg(unix)]
pub fn sql_password_from_env() -> Result<String, VarError> {
    std::env::var("PROD_SQL_PASSWORD")
}
