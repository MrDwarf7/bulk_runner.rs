#[cfg(target_os = "linux")]
#[cfg(not(target_os = "windows"))]
use std::env::VarError;

use deadpool_tiberius::tiberius::AuthMethod;

pub struct DbInfo {
    pub host: String,
    pub auth: AuthMethod,
    pub db:   String,
}

impl DbInfo {
    #[cfg(not(target_os = "windows"))]
    #[cfg(target_os = "linux")]
    pub fn from_env() -> Result<Self, VarError> {
        let host = crate::PROD_HOST.into();
        let db = crate::PROD_DB.into();
        let auth = sql_auth_method_from_env()?;
        Ok(Self { host, auth, db })
    }
}

// HACK: This is not supposed to be permenant - more a temporary solution to see if we can
/// pull the db info from the given file
#[cfg(target_os = "windows")]
#[cfg(not(target_os = "unix"))]
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

#[cfg(target_os = "windows")]
#[cfg(not(target_os = "unix"))]
impl Default for DbInfo {
    fn default() -> Self {
        Self {
            host: crate::PROD_HOST.into(),
            auth: AuthMethod::Integrated,
            db:   crate::PROD_DB.into(),
        }
    }
}

#[cfg(not(target_os = "windows"))]
#[cfg(target_os = "linux")]
impl From<String> for DbInfo {
    fn from(db_info: String) -> Self {
        let mut split = db_info.split_whitespace();
        let host = split.next().unwrap().to_string();
        let db = split.next().unwrap().to_string();

        let auth = sql_auth_method_from_env().expect("Failed to get SQL auth from env");
        Self { host, auth, db }
    }
}

#[cfg(not(target_os = "windows"))]
#[cfg(target_os = "linux")]
impl Default for DbInfo {
    fn default() -> Self {
        let tried = DbInfo::from_env();

        match tried {
            Ok(db_info) => db_info,
            Err(e) => panic!("Failed to get DbInfo from env: {}", e),
        }
    }
}

#[cfg(target_os = "linux")]
#[cfg(not(target_os = "windows"))]
pub fn sql_auth_method_from_env() -> Result<AuthMethod, VarError> {
    let user = sql_user_from_env()?;
    let password = sql_password_from_env()?;
    Ok(deadpool_tiberius::tiberius::AuthMethod::sql_server(user, password))
}

#[cfg(target_os = "linux")]
#[cfg(not(target_os = "windows"))]
pub fn sql_user_from_env() -> Result<String, VarError> {
    std::env::var("PROD_SQL_USER")
}

#[cfg(target_os = "linux")]
#[cfg(not(target_os = "windows"))]
pub fn sql_password_from_env() -> Result<String, VarError> {
    std::env::var("PROD_SQL_PASSWORD")
}
