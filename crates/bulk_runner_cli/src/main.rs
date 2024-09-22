use std::path::PathBuf;

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
    let cli = cli::Cli::new();
    let bot_semaphore = Arc::new(Semaphore::new(cli.total_bots));
    let sql_file = match cli.sql_file {
        Some(file) => file,
        None => PathBuf::from("bots.sql"),
    };
    let mut ctx: Context<DbInfo> = Context::async_from(sql_file.clone()).await;

    let bots: Vec<Bot> = query_database(&mut ctx, sql_file.clone()).await?;

    let mut futures: FuturesUnordered<AutomateCFuture> = FuturesUnordered::new();
    // let (tx, mut rx) = tokio::sync::mpsc::channel(bots.len());

    for bot in bots {
        // let tx = tx.clone();
        let permit = bot_semaphore.clone().acquire_owned().await?;
        let cmd = AutomateCBuilder::default()
            .with_sso()
            .with_process(&cli.process)
            .with_resource(&bot.name)
            .build();

        let future_cmd = cmd.run().await?;
        futures.push(future_cmd);

        drop(permit);

        // tokio::task::spawn(async move { cmd.run_async().await} }
        // tx.send((permit, future_cmd)).await.unwrap();
    }

    tokio::spawn(async move {
        let permit = bot_semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| Error::Generic(e.to_string()));
        let _ = run_bots(&mut futures).await;
        drop(permit);
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
    let mut outputs = Vec::new();
    while let Some(result) = futures.next().await {
        match result {
            Ok(output) => outputs.push(BotOutput::from(output)),
            Err(e) => outputs.push(BotOutput::from(e)),
        };
    }

    println!("Outputs:");
    for output in outputs {
        output.print_buffer();
    }

    Ok(())
}
