use deadpool_tiberius::tiberius::AuthMethod;

use crate::prelude::*;

use super::query_engine::QueryEngine;
use super::DbContext;

/// For passing the file and figuring out what it is based on what we're connecting to in the query.
#[derive(Debug, Clone)]
pub struct DbInfo {
    pub(crate) host: String,
    pub(crate) db: String,
    pub(crate) auth: AuthMethod,
}

impl DbContext for DbInfo {
    fn get_query_engine(&self) -> Box<QueryEngine> {
        Box::new(QueryEngine::from(self.clone()))
    }
}

impl DbInfo {
    pub fn new() -> Result<Self> {
        // TODO: Have the option to find the db cons in the sql file or env vars
        // let contents = match Self::try_parse_file(file) {
        //     Ok(contents) => contents,
        //     Err(e) => return Err(e),
        // };

        Ok(DbInfo::from_static())
    }

    fn from_static() -> Self {
        DbInfo {
            host: PROD_HOST.to_string(),
            db: PROD_DB.to_string(),
            auth: AuthMethod::Integrated,
        }
    }
}

// This is the section for parsing the file and trying to pull out the host & db from the sql file, it's a TODO atm
impl DbInfo {
    fn try_parse_file(file: PathBuf) -> Result<Self> {
        let contents = std::fs::read_to_string(file).expect("Failed to read file");
        let lines = contents.lines();

        let mut host = String::new();
        let mut db = String::new();

        // TODO: This is super janky lol
        // TODO:
        for line in lines {
            host = Self::isolate(line, "host")?;
            db = Self::isolate(line, "db")?;
        }

        let (host, db) = Self::check_empty(&host, &db).unwrap_or((PROD_HOST, PROD_DB));

        Ok(DbInfo {
            host: host.to_string(),
            db: db.to_string(),
            auth: AuthMethod::Integrated,
        })
    }

    // TODO: Make this more robust.
    // TODO:
    fn isolate(part_a: &str, part_b: &str) -> Result<String> {
        let mut buffer = String::new();

        if part_a.contains(part_b) {
            buffer = part_a.split("=").nth(1).unwrap().to_string();
        }

        Ok(buffer)
    }

    fn check_empty<'a>(host: &'a str, db: &'a str) -> Option<(&'a str, &'a str)> {
        if host.is_empty() && db.is_empty() {
            return Some((PROD_HOST, PROD_DB));
        }

        match (host, db) {
            ("", db) => Some((PROD_HOST, db)),
            (host, "") => Some((host, PROD_DB)),
            _ => Some((host, db)),
        }
    }
}
