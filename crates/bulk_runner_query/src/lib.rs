mod command_builder;
mod db_info;
mod dispatch;
mod error;
mod query_engine;

pub use crate::command_builder::AutomateBuilderBase;
pub use crate::db_info::DbInfo;
pub use crate::dispatch::{cli_dispatch, query_database};
pub use crate::query_engine::QueryEngine;

// use bulk_runner_bots::{BaseBot, Bot};

// use tokio::sync::mpsc::UnboundedSender;

pub use crate::error::Error;
use tracing::{error, info};
pub type Result<T> = std::result::Result<T, Error>;

// use crate::prelude::*;

// Need Result & Error types

// pub static DEFAULT_QUERY_FILE: &str = "bots.sql";

pub static PROD_HOST: &str = "PRDLGDB2";
pub static PROD_DB: &str = "BP_PRD";
