use futures::StreamExt;

use crate::cli::Cli;
use crate::prelude::*;
use crate::{Dispatchable, Packet};

pub struct Runner {
    process: String,
    concurrency_limit: usize,
    limit_total_runnable: usize,
    sql_file_contents: String,
}

impl From<Cli> for Runner {
    #[inline]
    fn from(cli: Cli) -> Self {
        Runner {
            process: cli.process().to_string(),
            concurrency_limit: cli.concurrency_limit(),
            limit_total_runnable: cli.limit_total_runnable(),
            sql_file_contents: cli.serialize_sql_file().unwrap_or("bots.sql".to_string()),
        }
    }
}

impl Runner {
    pub async fn run(&self) -> Result<()> {
        info!("->> {:<12}", "RUN:: Starting run");

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        // Serialize the sql file to a string
        let sql_file_contents = self.sql_file_contents.clone();

        let limit_total_runnable = self.limit_total_runnable;

        // Spawn a task to fetch the bots from the database,
        let query_handle = tokio::spawn(async move {
            // let tx = tx.clone();
            info!("->> {:<12}", "RUN::  Querying database...");
            bulk_runner_query::query_database(tx, sql_file_contents, limit_total_runnable).await;
        });

        // As the query runs, it will return back a Bot (which will have been filled already, we need the Bot to go to next step)
        let capacity = self.concurrency_limit;

        let future_bots = tokio::spawn(async move {
            let mut bots = Vec::with_capacity(capacity);
            while let Some(bot) = rx.recv().await {
                if bot.is_available().is_none() {
                    warn!("->> {:<12} - {:?}", "Future Bots:: Bot not available...", &bot);
                    break;
                }

                info!("->> {:<12} - {:?}", "Future Bots:: Bot received...", &bot);
                bots.push(bot.into_future());
            }
            rx.close();
            bots
        })
        .await?;

        let dispatchable: Dispatchable = futures::future::join_all(future_bots)
            .await
            .into_iter()
            .filter_map(|bot| bot.0)
            .map(|bot| Packet::new(bot, self.process.clone()))
            .collect::<Dispatchable>();

        query_handle.await?;

        bulk_runner_query::cli_dispatch(dispatchable.into(), self.concurrency_limit).await;

        Ok(())
    }
}
