/// # Bulk Runner
///
/// This is the main entry point for the Bulk Runner application.
/// It uses the `bulk_runner_rs` crate to run Blue Prism AutomateC processes in bulk.
///
/// ## Features
/// - `async` - Enable asynchronous execution of `AutomateC` commands.
///
/// Access to this is done via the `AutomateC` struct and the [automatec::AutomateC::run_async] method.
/// This uses the `tokio` crate to execute the commands asynchronously.
use bulk_runner_rs::*;
/// Re-export the `Error`, `Result`, and `W` types from the `bulk_runner_rs` crate
pub use bulk_runner_rs::{Error, Result, W};
use tracing::info;

use bulk_runner_rs::bot_handlers::{Bot, BotOutput};
use database::AsyncFrom;

/// # Main
///
/// This is the main function for the Bulk Runner application.
/// It creates a semaphore and a vector of bots, then creates a vector of futures for the bots.
/// It then runs the bots by executing the futures and collecting the outputs.
///
/// # Arguments
/// - process: The process to run on the bots.
/// - Optional: total_bots: The number of bots to run concurrently. Defaults to 30.
/// - Optional: sql_file: The path to the SQL file to pull the bots from. Defaults to "bots.sql".
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = cli::Cli::new();
    let bot_semaphore = Arc::new(Semaphore::new(cli.total_bots));

    let mut ctx: Context<DbInfo> = Context::async_from(cli.sql_file().clone()).await;

    let bots: Vec<Bot> = query_database(&mut ctx, cli.sql_file().clone()).await?;
    // let barrier = Arc::new(tokio::sync::Barrier::new(bots.len() + 1));

    let mut futures: FuturesUnordered<AutomateCFuture> = FuturesUnordered::new();
    // let (tx, mut rx) = tokio::sync::mpsc::channel(bots.len());

    for bot in bots {
        // let tx = tx.clone();
        info!("Running bot: {} with {}", bot.name, &cli.process);
        let permit = bot_semaphore.clone().acquire_owned().await?;
        let cmd = AutomateCBuilder::default()
            .with_sso()
            .with_process(&cli.process)
            .with_resource(&bot.name)
            .build();
        let future_cmd = cmd.run().await?;
        futures.push(future_cmd);
        drop(permit);
    }

    info!("Futures len: {}", futures.len());
    info!("First tasks finalized. Running bots...");

    tokio::spawn(async move {
        info!("Run bots inner task call...");
        // let b = barrier.clone();
        let permit = bot_semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| Error::Generic(e.to_string()));
        let _ = run_bots(&mut futures).await;

        // let wait = b.wait().await;
        drop(permit);
        // wait
    })
    .await?;

    // while let Some(result) = futures.
    // let automate = AutomateCBuilder::default()
    //     .with_sso()
    //     .with_process("AIM Members")
    //     .with_resource("ABC123")
    //     .build();
    //
    // dbg!(&automate);
    // println!("{}", automate);
    //
    // automate.run()?;

    Ok(())
}

/// # Run Bots
///
/// This function runs the bots by executing the futures and collecting the outputs.
///
/// # Arguments
/// - `futures`: A mutable reference to a `FuturesUnordered` of `AutomateCFuture` structs.
///
/// # Returns
/// - A `Result<()>` indicating the success or failure of the operation.
///
/// # Example
/// ```
/// let mut futures = FuturesUnordered::new();
/// let automate = AutomateCBuilder::default()
///     .with_sso()
///     .with_process("AIM Members")
///     .with_resource("ABC123")
///     .build();
///
/// let future_cmd = automate.run_async().await?;
/// futures.push(future_cmd);
///
/// let result = run_bots(&mut futures).await;
/// ```
///
/// # Errors
/// - If any of the futures fail, the function will return an error.
///
/// # Panics
/// - If the futures are not executed correctly, the function will panic.
///
#[allow(clippy::unused_async)]
async fn run_bots<'a>(futures: &mut FuturesUnordered<AutomateCFuture>) -> Result<()> {
    let mut outputs = Vec::with_capacity(futures.len());
    let mut futures = futures.fuse();
    let mut bot_output = BotOutput::default();

    info!("Inside run_bots -- Running futures...");

    while let Some(result) = futures.next().await {
        match result {
            Ok(output) => {
                bot_output.add_message(output.stdout.into_boxed_slice());
                outputs.push(())
            }
            Err(e) => {
                bot_output.add_message(e.to_string().into_boxed_str());
                outputs.push(())
            }
        };
    }

    // while let Some(result) = futures.next().await {
    //     match result {
    //         Ok(output) => outputs.push(BotOutput::from(output)),
    //         Err(e) => outputs.push(BotOutput::from(e)),
    //     };
    // }

    bot_output.print_buffer();

    Ok(())
}
