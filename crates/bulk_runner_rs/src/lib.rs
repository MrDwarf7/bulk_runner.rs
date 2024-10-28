// pub mod bot_handlers;
// pub mod database;

mod error;
pub mod timekeeper;

pub mod cli;
pub mod prelude;
pub mod runner;

pub use futures::{stream::FuturesUnordered, StreamExt};
pub use std::sync::Arc;
pub use tokio::sync::Semaphore;

pub use tracing::{debug, error, info, warn};

pub use crate::timekeeper::TimeKeeper;

// use bulk_runner_bots::{BaseBot, Bot};

// use bulk_runner_query::{AutomateBuilderBase, AutomateBuilderBaseExt};

pub use self::prelude::{Error, Result, W};

pub use self::runner::Runner;

pub type TracingSubscriber = tracing_subscriber::fmt::SubscriberBuilder<
    tracing_subscriber::fmt::format::DefaultFields,
    tracing_subscriber::fmt::format::Format<tracing_subscriber::fmt::format::Full>,
    tracing_subscriber::EnvFilter,
>;

// pub use bulk_runner_internals as internals;
// pub use internals::{AutomateCBuilder, AutomateCFuture};

// use bot_handlers::{Bot, Status};

// pub use database::{query_database, Context, DbInfo};

// pub use cli::Cli;
// pub use query::query_database;

// pub use tracing::error;
// pub use self::{BaseBot, Bot};
