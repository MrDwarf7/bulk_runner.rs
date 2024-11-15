use futures::StreamExt;

use crate::cli::Cli;

use crate::prelude::*;

use crate::{Dispatchable, Packet};

pub struct Runner {
    process: String,
    concurrency_limit: usize,
    total_run_on: usize,
    sql_file_contents: String,
}

impl From<Cli> for Runner {
    fn from(cli: Cli) -> Self {
        let process = cli.process().to_string();
        let concurrency_limit = cli.concurrency_limit();
        let total_run_on = cli.total_run_on();
        let sql_file_contents = cli.serialize_sql_file().unwrap_or("bots.sql".to_string());

        Runner {
            process,
            concurrency_limit,
            total_run_on,
            sql_file_contents,
        }
    }
}

impl Runner {
    pub async fn run(&self) -> Result<()> {
        info!("->> {:<12}", "RUN:: Starting run");

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        // Serialize the sql file to a string
        let sql_file_contents = self.sql_file_contents.clone();

        let total_run_on = self.total_run_on;

        // Spawn a task to fetch the bots from the database,
        let query_handle = tokio::spawn(async move {
            // let tx = tx.clone();
            info!("->> {:<12}", "RUN::  Querying database...");
            // TODO:
            // We want to then pass in -
            // self.total_run_on
            // as the param binding
            bulk_runner_query::query_database(tx, sql_file_contents, total_run_on).await;
        });

        // As the query runs, it will return back a Bot (which will have been filled already, we need the Bot to go to next step)
        let capacity = self.concurrency_limit;

        let future_bots = tokio::spawn(async move {
            let mut bots = Vec::with_capacity(capacity);
            while let Some(bot) = rx.recv().await {
                if bot.is_available().is_none() {
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

/* TODO: implement this

// struct that holds the VecDeque of bots and the process name to be dispatched
// Better way though -> Struct that IS itself a bot + process name
// ConstructedDispatch { bot: Bot, process_name: String }
//
// and we impl a from iterator onto it - to be able to collect from the exsiting future_bots
// that gets rid of the awkward stuff above
//
// Other thing we can do ->
//
// Move the awkward logic of cli_dispatch from there
// to here
//
// cli_dispatch(dispatch_bots: VecDeque<(Bot, String)>, total_bots: usize) -> Result<()>;
//
// then becomes
//
// cli_dispatch(packets: ConstructedDispatch, total_bots: usize) -> Result<()>;
//
// ->
// If we move each of the 'ConstructedDispatch' (ie: one thing we can send down the pipeline)
// // // VecDequeue<CosntructedDispatch>
// into a higher level struct we can remove another awkward part.
// DispatchablePackets { packets: VecDeque<ConstructedDispatch>, total_bots: usize }
// we then have access to all of it in one place.
//
// Then finally, we can have the cli_dispatch function handled by an orchestrator/controller
// that can handle the dispatching of the bots, and the handling of the results.
//
// CliDispatchController { semaphore: Arc<tokio::sync::Semaphore, localset: tokio::task::LocalSet }
// and provide a way to create a loop somewhere, and call the CliDispatchController with a method or assoc. function
//
// let controller = CliDispatchController::new(_______);
// while x < y {
//   controller.dispatch();
//  }
//
// dispatch itself would have access to semaphore via the length of the VecDeque, and the localset to actually spawn the tasks
// and then we can handle the results in a more controlled way.
//
// Actually - perhaps we can use a join set? It's no longer dictated by the fn scope itself, but by the controller (Which holds it's own state)
//
// This also removes the weird borderline circular dependency (almost)
//
//
//
//

/// One 'packet' that can be sent to the automatec.exe binary
struct ConstructedDispatch {
    bot: Bot,
    process_name: String,
}

/// Abstraction layer as a colleciton + length
struct DispatchablePackets {
    packets: VecDeque<ConstructedDispatch>,
    total_bots: usize,
}

impl From<UnboundedReceiver<Bot>> for DispatchablePackets {
    fn from(rx: UnboundedReceiver<Bot>) -> Self {
        let mut packets = VecDeque::with_capacity(40);
        while let Some(bot) = rx.recv().await {
            // Can move this looping part to an async fn
            if bot.is_available().is_none() {
                #[rustfmt::skip]
                warn!("->> {:<12} - {:?}", "Future Bots:: Bot not available...", &bot);
                break;
            }

            info!("->> {:<12} - {:?}", "Future Bots:: Bot received...", &bot);
            packets.push_back(ConstructedDispatch {
                bot,
                process_name: "process".to_string(),
            });
        }
        rx.close();
        DispatchablePackets {
            packets,
            total_bots: packets.len(),
        }
    }
}

/// Agnostic controller that can handle the dispatching of the bots
/// intake packets/data via funciton, not stored in the struct (Don't transfer ownership)
struct CliDispatchController {
    semaphore: Arc<tokio::sync::Semaphore>,
    localset: tokio::task::LocalSet,
}

impl CliDispatchController {
    fn new(semaphore: Arc<tokio::sync::Semaphore>, localset: tokio::task::LocalSet) -> Self {
        CliDispatchController {
            semaphore,
            localset,
        }
    }
}

 */
// TODO: implement this
