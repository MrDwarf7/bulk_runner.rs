use bulk_runner_bots::BaseBot;
use deadpool_tiberius::tiberius::{Query, Row};
use deadpool_tiberius::{Manager, Pool};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::db_info::DbInfo;
use crate::Result;

pub struct QueryEngine {
    pub(crate) pool: Pool,
}

#[cfg(windows)]
#[cfg(not(unix))]
impl Default for QueryEngine {
    /// Assembled a default DBInfo struct, and then creates a QueryEngine from it
    /// # Panics
    /// Panics if the QueryEngine cannot be created or the DBInfo cannot be created
    fn default() -> Self {
        QueryEngine::new(DbInfo::default()).expect("Failed to create QueryEngine")
    }
}

#[cfg(not(windows))]
#[cfg(unix)]
impl Default for QueryEngine {
    /// Assembled a default `DBInfo` struct, and then creates a `QueryEngine` from it
    ///
    /// # Panics
    ///
    /// Panics if the `QueryEngine` cannot be created or the `DBInfo` cannot be created
    fn default() -> Self {
        QueryEngine::new(DbInfo::auth_from_env().expect("Failed to create DbInfo from env"))
            .expect("Failed to create QueryEngine")
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

    /// Retrieves bots from the database based on the provided SQL query and limit.
    ///
    /// # Errors
    /// Returns an error if the query fails.
    pub async fn get_bots<S>(&self, parsed_file: S, limit_total_runnable: usize) -> Result<Vec<BaseBot>>
    where
        S: AsRef<str> + Send + Sync,
    {
        let limited_total_runnable = u8::try_from(if limit_total_runnable == 0 {
            u8::MAX as usize
        } else {
            limit_total_runnable
        })
        .unwrap_or(u8::MAX);

        Ok(self
            .query(parsed_file.as_ref(), limited_total_runnable)
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
    async fn query<S>(&self, query: S, limit_total_runnable: u8) -> Result<Vec<Row>>
    where
        S: AsRef<str> + Send + Sync,
    {
        #[rustfmt::skip]
        let mut con = self.pool.get().await.expect("Failed to get pooled connection in run_query");

        let mut results = Query::new(query.as_ref());
        results.bind(limit_total_runnable);

        let results = results.query(&mut con).await?.into_results().await?;

        Ok(results.into_par_iter().flat_map(|row| row).collect::<Vec<Row>>())
    }
}

impl From<DbInfo> for QueryEngine {
    fn from(value: DbInfo) -> Self {
        QueryEngine::new(value).expect("Failed to create QueryEngine")
    }
}
