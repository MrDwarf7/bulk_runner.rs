use bulk_runner_rs::*;

pub use bulk_runner_rs::{Error, Result, W};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();

    let time = TimeKeeper::new();

    info!("1. Starting bulk_runner_rs...");

    info!("->> {:<12}", "MAIN:: 1. Starting bulk_runner_rs... ");
    let cli = cli::Cli::new();
    info!("->> {:<12}", "MAIN:: 2. Cli initialized... ");

    if let Err(e) = Runner::from(cli).run().await {
        error!("->> {:<12} - {}", "MAIN:: 3. Error running cli... ", e);
        std::process::exit(1);
    }

    time.print_elapsed();
    time.print_started_at();

    Ok(())
}
