use bulk_runner_rs_v2::*;

pub use bulk_runner_rs_v2::{Error, Result, W};
use prelude::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();

    info!("1. Starting bulk_runner_rs_v2...");

    info!("->> {:<12}", "MAIN:: 1. Starting bulk_runner_rs_v2... ");
    let cli = cli::Cli::new();
    info!("->> {:<12}", "MAIN:: 2. Cli initialized... ");

    if let Err(e) = cli.run().await {
        error!("->> {:<12} - {}", "MAIN:: 3. Error running cli... ", e);
        std::process::exit(1);
    }

    Ok(())
}
