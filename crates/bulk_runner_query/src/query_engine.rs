use bulk_runner_bots::BaseBot;
use deadpool_tiberius::tiberius::{Query, Row};
use deadpool_tiberius::{Manager, Pool};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::db_info::DbInfo;
use crate::{BulkRunnerQueryError, Result};

pub struct QueryEngine {
    pub(crate) pool: Pool,
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
    ///
    /// Returns an error if the query fails.
    pub async fn get_bots<S>(&self, parsed_file: S, limit_total_runnable: u8) -> Result<Vec<BaseBot>>
    where
        S: AsRef<str> + Send + Sync,
    {
        let limited_total_runnable = if limit_total_runnable == 0 {
            u8::MAX
        } else {
            limit_total_runnable
        };

        Ok(self
            .query(parsed_file.as_ref(), limited_total_runnable)
            .await?
            .par_iter()
            .map(BaseBot::from)
            .collect::<Vec<BaseBot>>())
    }
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
        let mut con = self.pool.get().await.map_err(|e| {
            BulkRunnerQueryError::ConnectionError(format!("Failed to get connection from pool: {e}"))
        })?;

        let mut results = Query::new(query.as_ref());
        results.bind(limit_total_runnable);

        let results = results.query(&mut con).await?.into_results().await?;

        Ok(results.into_par_iter().flat_map(|row| row).collect::<Vec<Row>>())
    }
}

impl TryFrom<DbInfo> for QueryEngine {
    type Error = BulkRunnerQueryError;

    fn try_from(value: DbInfo) -> Result<Self> {
        Self::new(value)
    }
}
