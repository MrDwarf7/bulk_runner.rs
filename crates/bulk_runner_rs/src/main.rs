use bulk_runner_rs::*;
pub use bulk_runner_rs::{Error, Result, W};

#[tokio::main]
async fn main() -> Result<()> {
    let timekeep = TimeKeeper::default();
    let cli = cli::Cli::new().check_automate_exists()?;
    init_logger(cli.verbosity_level().into()).init();

    info!("1. Starting bulk_runner_rs...");

    info!("->> {:<12}", "MAIN:: 1. Starting bulk_runner_rs... ");
    info!("->> {:<12}", "MAIN:: 2. Cli initialized... ");

    if let Err(e) = Runner::from(cli).run().await {
        error!("->> {:<12} - {}", "MAIN:: 3. Error running cli... ", e);
        std::process::exit(1);
    }

    timekeep.print_elapsed();
    timekeep.print_started_at();

    Ok(())
}

fn init_logger(level: tracing_subscriber::filter::EnvFilter) -> TracingSubscriber {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_env_filter(level)
    // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
    // .with_timer(tracing_subscriber::fmt::time::SystemTime)
}
