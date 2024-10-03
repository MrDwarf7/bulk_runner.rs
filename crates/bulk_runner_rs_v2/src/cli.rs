use crate::{Bot, Result};
use clap::{command, Parser};
use futures::StreamExt;
// use std::collections::HashMap;
use dashmap::DashMap as HashMap;
// use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::*;

use crate::prelude::*;

#[derive(Parser, Debug)]
#[command(
    name = "BulkRunner.rs",
    about = "A CLI tool to run Blue Prism processes (Via AutomateC dispatch) in bulk.",
    long_about = "\n
    BulkRunner.rs is a CLI tool designed to facilitate the execution of Blue Prism AutomateC processes in bulk.
    It streamlines the process of launching multiple instances/process via the Control Room, each targeting a distinct resource or bot.
    This is particularly useful during change over periods where multiple bots need to be transitioned from one process to another.",
    version,
    arg_required_else_help = true
)]
pub struct Cli {
    /// The process to run on all the bots pulled by the SQL query.
    #[arg(
        //
        index = 1,
        help = "The process to run the bots on.",
        value_hint = clap::ValueHint::Other
    )]
    pub process: String,

    /// The number of bots to run concurrently.
    /// This is handled internally via a semaphore, not via the hardware concurrency of the CPU.
    #[arg(
        short = 't',
        long = "total",
        default_value = "30",
        //index = 2,
        long_help = "The number of bots to run concurrently. This is handled internally via a semaphore, not via the hardware concurrency of the CPU.",
        value_hint = clap::ValueHint::Other
    )]
    pub total_bots: usize,

    /// Optional path to a SQL file to pull the bots from.
    /// If not provided, the default value is "bots.sql".
    /// And is looked for in the current working directory of the binary.
    #[arg(
        short = 'f',
        long = "file",
        help = "The path to the SQL file.",
        default_value = "bots.sql",
        value_hint = clap::ValueHint::FilePath
    )]
    sql_file: Option<PathBuf>,
}

fn check_automate_exists() -> Result<()> {
    if std::env::var("BYPASS_AUTOMATEC_CHECK").is_ok() {
        return Ok(());
    }
    let path = std::path::Path::new(&*DEFAULT_EXE_PATH);
    if !path.exists() {
        return Err(Error::Generic(format!(
            "AutomateC does not exist at path: {:?}",
            path
        )));
    }
    Ok(())
}

impl Cli {
    /// Create a new instance of the Cli struct.
    ///
    /// # Notes:
    /// This will check if the AutomateC executable exists at the path specified in the prelude.
    /// If it does not exist, it will return an error and exit the process.
    ///
    /// There is a bypass for this check, which can be set by setting the environment variable BYPASS_AUTOMATEC_CHECK.
    ///
    /// This is useful for testing purposes.
    pub fn new() -> Self {
        match check_automate_exists() {
            Ok(_) => (),
            Err(e) => {
                error!("Error: {}", e);
                std::process::exit(1);
            }
        }

        let mut s = Self::parse();
        if s.sql_file.is_none() {
            s.sql_file = Some(PathBuf::from("bots.sql"));
        }
        s
    }
    pub fn process(&self) -> &str {
        &self.process
    }

    pub fn total_bots(&self) -> usize {
        self.total_bots
    }

    pub fn sql_file(&self) -> &PathBuf {
        self.sql_file.as_ref().unwrap()
    }
}

impl Cli {
    pub async fn serialize_sql_file(&self) -> Result<String> {
        info!("SERIALIZE:: Starting serialization...");
        let buffer = std::fs::read(self.sql_file())?;
        let sql_file_query = String::from_utf8(buffer)?.replace("\n", " ");
        // let sql_file_query = sql_file_query.replace("\n", " ");

        Ok(sql_file_query)
    }

    pub async fn run(&self) -> Result<()> {
        // Create a 'generic base' to fill in later;
        info!("->> {:<12}", "RUN:: Starting run");

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        // Serialize the sql file to a string
        let parsed_sql_file = self.serialize_sql_file().await?;
        // info!("RUN:: Parsed SQL file: {:#?}", &parsed_sql_file);
        info!(
            "->> {:<12} - {:#?}",
            "RUN:: Parsed SQL file", &parsed_sql_file
        );
        // Spawn a task to fetch the bots from the database,
        // We can then fill the base_bot with the new data that's come back from the query [name && current_status]
        tokio::spawn(async move {
            let tx = tx.clone();

            info!("->> {:<12}", "RUN::  Querying database...");
            crate::query_controller::query_database(tx, &parsed_sql_file).await;
        })
        .await?;

        // As the query runs, it will return back a Bot (which will have been filled already, we need the Bot to go to next step)
        let future_bots = tokio::spawn(async move {
            let mut bots = Vec::with_capacity(40);
            while let Some(bot) = rx.recv().await {
                if bot.if_available().is_none() {
                    warn!(
                        "->> {:<12} - {:?}",
                        "Future Bots:: Bot not available...", &bot
                    );
                    break;
                }

                info!("->> {:<12} - {:?}", "Future Bots:: Bot received...", &bot);
                bots.push(bot.into_future());

                // if let Some(bot) = bot.if_available() {
                // } else {
                //     warn!(
                //         "->> {:<12} - {:?}",
                //         "Future Bots:: Bot not available...", &bot
                //     );
                //     break;
                // }

                // info!("->> {:<12} - {:?}", "Future Bots:: Bot received...", &bot);
                // bots.push(bot.into_future());
            }
            rx.close();
            bots
        })
        .await?;

        let filled_bots: Vec<Bot> = futures::future::join_all(future_bots)
            .await
            .into_iter()
            .map(|bot| {
                warn!("->> {:<12} - {:?}", "FULL bot:: filled_bots", &bot);
                match bot.0.is_some() {
                    true => bot.0.unwrap(),
                    false => bot.1,
                }
                //    bot.1
            })
            // .filter_map(|bot| {
            //     // TODO: Need to check this is the right column ret of the 2
            //
            //     warn!("Bot: {:?}", &bot);
            //     warn!("Bot 0 tuple: {:?}", &bot.0);
            //     warn!("Bot 1 tuple: {:?}", &bot.1);
            //
            //     bot.1
            // })
            .collect();

        info!("->> {:<12} - {:?}", "RUN:: Filled bots", &filled_bots);

        let dispatch_bots: HashMap<String, Bot> = filled_bots
            .par_iter()
            .map(|bot| (self.process.clone(), bot.clone()))
            .collect::<Vec<_>>() // Collect into a Vec first
            .into_iter()
            .collect();
        // Then convert to DashMap
        // });
        // iter().fold(HashMap::new(), |acc, bot| {
        //     acc.insert(self.process.clone(), bot.clone());
        //     acc
        // });

        crate::query_controller::cli_dispatch(dispatch_bots, self.total_bots()).await?;

        Ok(())
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

//
// lowest type
// Spec::fetch() | | | -> QueryResults{ base_bot: BaseBot::default() }
//
//
//
//  Cli -> Into::Into(BaseBot) Y
//    1.  waiting for ret from db to fill ... ... ...
//
// Query -> Get bots (using the sql file || we use a struct OF the sql file (Which we can serialize)
// at start?)
//
//  QueryResults -> Bot::from(QueryResults) BaseBot::into(Bot){This will 'fill' copies of the base
//  bot with the new data that's come back from the query [name && current_status]}
//  1 -> Bot::from(QueryResults) -> Bot
//
//
