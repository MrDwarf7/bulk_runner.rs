use deadpool_tiberius::{
    tiberius::{Query, Row},
    Manager, Pool,
};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use super::DbInfo;
use crate::{prelude::*, BaseBot};

pub struct QueryEngine {
    pub(super) pool: Pool,
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

    pub async fn get_bots(&mut self, parsed_file: &str) -> Result<Vec<BaseBot>> {
        Ok(self
            .run_query(parsed_file)
            .await?
            .par_iter_mut()
            .map(BaseBot::from)
            .collect::<Vec<BaseBot>>())
    }

    async fn run_query(&mut self, query: &str) -> Result<Vec<Row>> {
        #[rustfmt::skip]
        let mut con = self.pool.get().await
            .expect("Failed to get pooled connection in run_query");

        #[rustfmt::skip]
        let results = Query::new(query).query(&mut con).await?.into_results().await?;

        Ok(results
            .into_par_iter()
            .flat_map(|row| row)
            .collect::<Vec<Row>>())
    }
}

impl From<DbInfo> for QueryEngine {
    fn from(value: DbInfo) -> Self {
        QueryEngine::new(value).expect("Failed to create QueryEngine")
    }
}
