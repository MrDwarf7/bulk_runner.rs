use std::collections::VecDeque;

use futures::StreamExt;

use crate::cli::Cli;
// use crate::{dispatcher, prelude::*, Bot};

use crate::prelude::*;
use bulk_runner_bots::Bot;

pub struct Runner {
    process: String,
    total_bots: usize,
    sql_file_contents: String,
}

impl From<Cli> for Runner {
    fn from(cli: Cli) -> Self {
        let process = cli.process().to_string();
        let total_bots = cli.total_bots();
        let sql_file_contents = cli.serialize_sql_file().unwrap_or("bots.sql".to_string());

        Runner {
            process,
            total_bots,
            sql_file_contents,
        }
    }
}

impl Runner {
    pub async fn run(&self) -> Result<()> {
        info!("->> {:<12}", "RUN:: Starting run");

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        // Serialize the sql file to a string
        // let parsed_sql_file = self.serialize_sql_file()?;
        let sql_file_contents = self.sql_file_contents.clone();
        #[rustfmt::skip]
        info!("->> {:<12} - {:#?}", "RUN:: Parsed SQL file", &sql_file_contents);
        // Spawn a task to fetch the bots from the database,
        // We can then fill the base_bot with the new data that's come back from the query [name && current_status]
        let query_handle = tokio::spawn(async move {
            let tx = tx.clone();
            info!("->> {:<12}", "RUN::  Querying database...");
            bulk_runner_query::query_database(tx, sql_file_contents).await;
        });

        // As the query runs, it will return back a Bot (which will have been filled already, we need the Bot to go to next step)
        let future_bots = tokio::spawn(async move {
            // TODO: STREAM: We can stream here also
            let mut bots = Vec::with_capacity(40);
            while let Some(bot) = rx.recv().await {
                if bot.if_available().is_none() {
                    #[rustfmt::skip]
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

        // NOTE: This is an into_future -> We can stream this via try_next(), what can we do with this?
        let mut filled_bots: Vec<Bot> = futures::future::join_all(future_bots)
            .await
            .into_iter()
            //
            .filter_map(|bot| bot.0)
            .collect();

        info!("->> {:<12} - {:?}", "RUN:: Filled bots", &filled_bots);

        let mut dispatch_bots: VecDeque<(Bot, String)> = VecDeque::with_capacity(filled_bots.len());
        filled_bots
            .drain(..)
            .for_each(|bot| dispatch_bots.push_back((bot, self.process.clone())));

        query_handle.await?;

        bulk_runner_query::cli_dispatch(dispatch_bots, self.total_bots).await;

        // dispatcher::cli_dispatch(dispatch_bots, self.total_bots).await?;

        Ok(())
    }
}
