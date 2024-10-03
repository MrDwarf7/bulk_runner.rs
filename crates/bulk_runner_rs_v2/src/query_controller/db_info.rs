use deadpool_tiberius::tiberius::AuthMethod;

pub struct DbInfo {
    pub host: String,
    pub auth: AuthMethod,
    pub db: String,
}

// HACK: This is not supposed to be permenant - more a temporary solution to see if we can
/// pull the db info from the given file
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

impl Default for DbInfo {
    fn default() -> Self {
        Self {
            host: crate::prelude::PROD_HOST.into(),
            auth: AuthMethod::Integrated,
            db: crate::prelude::PROD_DB.into(),
        }
    }
}
