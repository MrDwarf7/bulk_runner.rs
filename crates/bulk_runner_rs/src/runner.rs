use futures::StreamExt;

use crate::cli::Cli;
use crate::prelude::*;
use crate::{Dispatchable, Packet};

pub struct Runner {
    process:              String,
    // HACK: We honestly (probably) get rid of both of these fields... Or even do away with the
    // entire Runner struct and just have `fn run(...)` as a free-floating function that takes
    // either CLI, or the 3 external fields it needs honestly.
    concurrency_limit:    usize,
    limit_total_runnable: usize,
}

impl TryFrom<&Cli> for Runner {
    type Error = String;

    #[inline]
    fn try_from(cli: &Cli) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            process:              cli.process.clone(),
            concurrency_limit:    cli.concurrency_limit,
            limit_total_runnable: cli.limit_total_runnable,
        })
    }
}

impl Runner {
    // TODO: may be better to atually return an Option<()> here?

    /// Orchestrates the entire bulk runner process.
    ///
    /// # Errors
    /// Can fail if any step in the process encounters an error.
    /// We do our best-effort to recover, and failing that we log the error and continue.
    ///
    /// # Panics
    /// Panics if the SQL file cannot be read, as this is a critical failure that prevents the runner from functioning.
    pub async fn run<S>(&self, sql_file_content: S) -> Result<()>
    where
        S: AsRef<str> + Send + 'static,
    {
        info!("->> {:<12}", "RUN:: Starting run");

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        let limit_total_runnable = self.limit_total_runnable;

        // Spawn a task to fetch the bots from the database,
        let query_handle = tokio::spawn(async move {
            info!("->> {:<12}", "RUN::  Querying database...");
            bulk_runner_query::query_database(tx, sql_file_content, limit_total_runnable).await;
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

        let process = Box::leak(Box::new(self.process.clone()));

        let dispatchable: Dispatchable = futures::future::join_all(future_bots)
            .await
            .into_iter()
            .filter_map(|bot| bot.0)
            .map(|bot| Packet::new(bot, process.to_owned()))
            .collect::<Dispatchable>();

        query_handle.await?;

        bulk_runner_query::cli_dispatch(dispatchable.into(), self.concurrency_limit).await;

        Ok(())
    }
}
