use bulk_runner_rs::{Runner, TimeKeeper, TracingSubscriber};
use tracing::{error, info};

#[tokio::main]
async fn main() -> bulk_runner_rs::Result<()> {
    let timekeep = TimeKeeper::default();
    let cli = bulk_runner_rs::cli::Cli::new()?;
    init_logger(cli.verbosity_level.into()).init();

    info!("->> {:<12}", "MAIN:: 1. Starting bulk_runner_rs... ");
    info!("->> {:<12}", "MAIN:: 2. Cli initialized... ");

    let sql_file_contents = bulk_runner_rs::cli::read_sql_file(&cli.sql_file)
        .map_err(|e| error!("->> {:<12} {}", "MAIN::  Failed to read SQL file: ", e))
        .expect("Failed to read SQL file");

    Runner::from(&cli)
        .run(sql_file_contents)
        .await
        .map_err(|e| error!("->> {:<12} {}", "MAIN:: 4. Runner failed to run: ", e))
        .ok();

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
