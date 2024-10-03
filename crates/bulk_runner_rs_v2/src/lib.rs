// pub mod bot_handlers;
// pub mod database;

mod error;
pub mod prelude;

mod bots;
pub mod cli;
mod query_controller;

pub use futures::{stream::FuturesUnordered, StreamExt};
pub use std::sync::Arc;
pub use tokio::sync::Semaphore;
pub use tracing::error;

pub use self::bots::{BaseBot, Bot};

pub use self::prelude::{Error, Result, W};

// pub use bulk_runner_internals as internals;
// pub use internals::{AutomateCBuilder, AutomateCFuture};

// use bot_handlers::{Bot, Status};
//
// pub use database::{query_database, Context, DbInfo};

// pub use cli::Cli;
// pub use query::query_database;
