use std::sync::Arc;

use bulk_runner_bots::Bot;
use tokio::sync::mpsc::UnboundedSender;

use crate::query_engine::QueryEngine;
use crate::{error, info, DbInfo, Result};

/// Querys the database for bots to run based on the provided SQL file and sends them through the provided channel.
///
/// # Panics
/// Panics if the SQL file cannot be read, as this is a critical failure that prevents the runner from functioning.
///
pub async fn query_database<S: AsRef<str>>(
    tx: UnboundedSender<Bot>,
    parsed_sql_file: S,
    limit_total_runnable: u8,
) {
    let db_info = DbInfo::auth_from_env()
        .map_err(|e| error!("->> {:<12} - {:?}", "DB_INFO:: ERROR", e))
        .expect("Failed to create DbInfo from env");

    let mut base_bots = match QueryEngine::new(db_info) {
        Ok(engine) => {
            match engine
                .get_bots(parsed_sql_file.as_ref(), limit_total_runnable)
                .await
            {
                Ok(bots) => bots,
                Err(e) => {
                    error!("->> {:<12} - {:?}", "QUERY_ENGINE:: ERROR", e);
                    vec![]
                }
            }
        }
        Err(e) => {
            error!("->> {:<12} - {:?}", "QUERY_ENGINE:: ERROR", e);
            vec![]
        }
    };

    // TODO: STREAM: We can stream these instead of iterating over them?
    while let Some(base_bot) = base_bots.pop() {
        let filled_bot: Bot = Bot::from(base_bot);
        match filled_bot.status {
            bulk_runner_bots::BotStatus::Ready(ref status) => {
                info!("{:<12} - {:?}", "QUERY:: Ready bot", status);
                tx.send(filled_bot).unwrap_or_default();
            }
            bulk_runner_bots::BotStatus::NotReady(status) => {
                info!("{:<12} - {:?}", "QUERY:: Not ready bot", status);
            }
        }
    }
    drop(tx);
}

pub async fn cli_dispatch(mut dispatch_bots: Vec<(Bot, String)>, total_bots: usize) {
    let sempahore = Arc::new(tokio::sync::Semaphore::new(total_bots));

    let (dispatched_tx, mut dispatched_rx) = tokio::sync::mpsc::unbounded_channel();

    let blocking_task_handles = dispatch_bots
        .iter_mut()
        .map(|(bot, process_name)| {
            let sempahore = sempahore.clone();

            let bot = bot.clone();
            let process_name = process_name.clone();
            let dispatched_tx = dispatched_tx.clone();

            tokio::task::spawn_blocking(move || {
                let res = threaded_dispatch(&bot, &process_name, sempahore.as_ref());
                dispatched_tx.send(res).unwrap_or_default();
                drop(sempahore);
                drop(dispatched_tx);
            })
        })
        .collect::<Vec<_>>();

    let t1 = tokio::spawn(async move {
        for v in blocking_task_handles {
            v.await.map_err(error::Error::TokioJoinError).unwrap_or_default();
        }
    });
    drop(dispatched_tx);

    let t2 = tokio::spawn(async move {
        while let Some(res) = dispatched_rx.recv().await {
            match res {
                Ok(()) => info!("->> {:<12} - {}", "DISPATCH:: OK", "Bot ran successfully!"),
                Err(e) => error!("->> {:<12} - {:?}", "DISPATCH:: ERROR", e),
            }
        }
        dispatched_rx.close();
        drop(dispatched_rx);
    });

    let _ = futures::future::join_all(vec![t1, t2]).await;
}

#[tokio::main]
async fn threaded_dispatch(bot: &Bot, process_name: &str, sempahore: &tokio::sync::Semaphore) -> Result<()> {
    info!("->> {:<12} - {}: {}", "THREADED_DISP:: ", "Spawn local", &process_name);
    let permit = sempahore.acquire().await?;
    let commander = crate::command_builder::AutomateBuilderBase::default()
        .with_sso()
        .with_process(process_name)
        .with_resource(&bot.name)
        .build();

    let res = bulk_runner_bots::dispatch(bot.name.clone(), commander.into()).await;
    tokio::task::yield_now().await;
    check_err(res);
    drop(permit);

    Ok(())
}

pub fn check_err(res: bulk_runner_bots::Result<()>) {
    match res {
        Ok(()) => info!("->> {:<12} - {}", "CHECK_ERR:: OK", "Bot ran successfully!"),
        Err(e) => error!("->> {:<12} - {:?}", "CHECK_ERR:: ERROR", e),
    }
}
