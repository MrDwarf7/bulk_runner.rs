mod command_builder;
mod db_info;
mod dispatch;
mod error;
mod query_engine;

use tracing::{error, info};

pub use crate::command_builder::AutomateBuilderBase;
#[cfg(windows)]
#[cfg(not(unix))]
pub use crate::db_info::DbInfo;
#[cfg(not(windows))]
#[cfg(unix)]
pub use crate::db_info::{sql_password_from_env, sql_user_from_env, DbInfo};
pub use crate::dispatch::{cli_dispatch, query_database};
// use bulk_runner_bots::{BaseBot, Bot};

// use tokio::sync::mpsc::UnboundedSender;
pub use crate::error::Error;
pub use crate::query_engine::QueryEngine;
pub type Result<T> = std::result::Result<T, Error>;

// use crate::prelude::*;

// Need Result & Error types

// pub static DEFAULT_QUERY_FILE: &str = "bots.sql";

pub static PROD_HOST: &str = "PRDLGDB2";
pub static PROD_DB: &str = "BP_PRD";
