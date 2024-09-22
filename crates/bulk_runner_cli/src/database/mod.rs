mod db_context;
mod db_info;
pub mod query_engine;

pub use db_context::Context;
pub use db_info::DbInfo;
use query_engine::QueryEngine;

use crate::prelude::*;
use crate::Bot;

pub trait DbContext: Send + Sync {
    fn get_query_engine(&self) -> Box<QueryEngine>;
}

pub trait AsyncFrom<T> {
    type Output;
    fn async_from(value: T) -> impl std::future::Future<Output = Self::Output> + Send;
}

pub trait AsyncInto<T> {
    type Output;
    fn into(self) -> impl std::future::Future<Output = Self::Output> + Send;
}

// TODO: Create the full query and launch from sql file call
#[allow(clippy::unused_async)]
#[allow(clippy::missing_docs_in_private_items, clippy::missing_errors_doc)]
pub async fn query_database<T: DbContext>(
    ctx: &mut Context<T>,
    maybe_file: PathBuf,
) -> Result<Vec<Bot>> {
    let mut query_engine = ctx.get_query_engine();
    let bots = query_engine.get_bot_list(maybe_file).await?;
    let bots = bots
        .into_iter()
        .filter_map(|(name, status)| {
            if let crate::bot_handlers::Status::Idle { .. } = status {
                Some(Bot::new(name, status))
            } else {
                None
            }
        })
        // .map(|(name, status)| Bot::new(name, status))
        .collect();

    Ok(bots)
}
