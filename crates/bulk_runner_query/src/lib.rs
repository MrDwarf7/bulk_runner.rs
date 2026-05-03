mod command_builder;
mod db_info;
mod dispatch;
mod error;
mod query_engine;

use tracing::{error, info};

pub use crate::command_builder::AutomateBuilderBase;
pub use crate::db_info::DbInfo;
pub use crate::dispatch::{cli_dispatch, query_database};
pub use crate::error::Error as BulkRunnerQueryError;
//
pub use crate::query_engine::QueryEngine;
pub type Result<T> = std::result::Result<T, BulkRunnerQueryError>;
#[allow(unused_imports)]
pub(crate) use crate::error::Error;

pub static PROD_HOST: &str = "PRDLGDB2";
pub static PROD_DB: &str = "BP_PRD";
