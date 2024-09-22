pub mod bot_handlers;
mod error;
mod prelude;

pub mod cli;
pub mod database;

pub use futures::{stream::FuturesUnordered, StreamExt};
pub use std::sync::Arc;
pub use tokio::sync::Semaphore;

pub use bulk_runner_internals as internals;
pub use internals::{AutomateCBuilder, AutomateCFuture};

pub use self::prelude::{Error, Result, W};
use bot_handlers::{Bot, Status};

pub use database::{query_database, Context, DbInfo};

// pub use cli::Cli;
// pub use query::query_database;
