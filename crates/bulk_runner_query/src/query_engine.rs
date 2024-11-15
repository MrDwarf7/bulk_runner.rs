use deadpool_tiberius::{
    tiberius::{Query, Row},
    Manager, Pool,
};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::db_info::DbInfo;
use crate::Result;
use bulk_runner_bots::BaseBot;
// use crate::{BaseBot, Result};

pub struct QueryEngine {
    pub(crate) pool: Pool,
}

impl Default for QueryEngine {
    /// Assembled a default DBInfo struct, and then creates a QueryEngine from it
    /// # Panics
    /// Panics if the QueryEngine cannot be created or the DBInfo cannot be created
    fn default() -> Self {
        QueryEngine::new(DbInfo::default()).expect("Failed to create QueryEngine")
    }
}

impl QueryEngine {
    pub(crate) fn new(db_info: DbInfo) -> Result<Self> {
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

    pub async fn get_bots<S>(&self, parsed_file: S, total_run_on: u8) -> Result<Vec<BaseBot>>
    where
        S: AsRef<str> + Send + Sync,
    {
        Ok(self
            .query(parsed_file.as_ref(), total_run_on)
            .await?
            .par_iter()
            .map(BaseBot::from)
            .collect::<Vec<BaseBot>>())
    }

    // Add pub methods here to access the run_query method
    // returned data will likely need to impl From<Row> for YourStruct
}

#[async_trait::async_trait]
pub trait Queryable {
    async fn query<S>(&self, query: S, total_run_on: u8) -> Result<Vec<Row>>
    where
        S: AsRef<str> + Send + Sync;
}

#[async_trait::async_trait]
impl Queryable for QueryEngine {
    async fn query<S>(&self, query: S, total_run_on: u8) -> Result<Vec<Row>>
    where
        S: AsRef<str> + Send + Sync,
    {
        #[rustfmt::skip]
        let mut con = self.pool.get().await
            .expect("Failed to get pooled connection in run_query");

        #[rustfmt::skip]
        let mut results = Query::new(query.as_ref());
        results.bind(total_run_on);

        let results = results.query(&mut con).await?.into_results().await?;

        Ok(results
            .into_par_iter()
            .flat_map(|row| row)
            .collect::<Vec<Row>>())
    }
}

// impl QueryEngine {
//     async fn run_query(&self, query: impl AsRef<str>) -> Result<Vec<Row>> {
//         #[rustfmt::skip]
//         let mut con = self.pool.get().await
//             .expect("Failed to get pooled connection in run_query");

//         #[rustfmt::skip]
//         let results = Query::new(query.as_ref()).query(&mut con).await?.into_results().await?;

//         Ok(results
//             .into_par_iter()
//             .flat_map(|row| row)
//             .collect::<Vec<Row>>())
//     }
// }

impl From<DbInfo> for QueryEngine {
    fn from(value: DbInfo) -> Self {
        QueryEngine::new(value).expect("Failed to create QueryEngine")
    }
}
