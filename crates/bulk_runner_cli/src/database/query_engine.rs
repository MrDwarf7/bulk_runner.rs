use deadpool_tiberius::{
    tiberius::{Query, Row},
    Manager, Pool,
};
use std::collections::HashMap;

use super::DbInfo;
use crate::{bot_handlers::Status, prelude::*};

pub struct QueryEngine {
    pub(super) pool: Pool,
}

impl QueryEngine {
    pub(in crate::database) fn new(db_info: DbInfo) -> Result<Self> {
        let pool = Manager::new()
            .host(db_info.host)
            .authentication(db_info.auth)
            .trust_cert()
            .database(db_info.db)
            .max_size(8)
            .wait_timeout(2)
            .recycle_timeout(8)
            // .create_timeout(5.0)
            .create_pool()?;

        Ok(Self { pool })
    }

    pub async fn get_bot_list(&mut self, file: PathBuf) -> Result<HashMap<String, Status>> {
        let query = Self::serialize_sql_file(file)?;
        let rows = self.run_query(&query).await?;
        let map = Self::generate_map(rows);
        Ok(map)
    }

    async fn run_query(&mut self, query: &str) -> Result<Vec<Row>> {
        let mut rows = Vec::new();
        let mut con = self
            .pool
            .get()
            .await
            .expect("Failed to get pooled connection in run_query");

        let query = Query::new(query);
        let results = query.query(&mut con).await?.into_results().await?;

        results.into_iter().flatten().for_each(|row| {
            rows.push(row);
        });

        Ok(rows)
    }

    fn generate_map(rows: Vec<Row>) -> HashMap<String, Status> {
        let mut map = HashMap::new();

        // Column 0 is the name, column 1 is the status
        for row in rows {
            let name: String = row
                .try_get(0)
                .unwrap_or(Some("Unknown"))
                .unwrap()
                .to_string();

            let status_str: &str = row.get(1).unwrap_or("Unknown");
            let status = match status_str {
                "Idle" => Status::Idle,
                "Pending" => Status::Pending,
                "Running" => Status::Running,
                "Completed" => Status::Completed,
                "Failed" => Status::Failed,
                _ => Status::Unknown,
            };
            map.insert(name, status);
        }
        map
    }

    fn serialize_sql_file(file: PathBuf) -> Result<String> {
        let file_contents = std::fs::read_to_string(file).expect("Failed to read sql file");
        let file_contents = file_contents.replace("\n", " ");
        Ok(file_contents)
    }
}

impl From<DbInfo> for QueryEngine {
    fn from(value: DbInfo) -> Self {
        QueryEngine::new(value).expect("Failed to create QueryEngine")
    }
}
