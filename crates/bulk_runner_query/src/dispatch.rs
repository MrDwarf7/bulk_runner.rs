use std::collections::VecDeque;
use std::sync::Arc;

use tokio::sync::mpsc::UnboundedSender;

use bulk_runner_bots::{BaseBot, Bot};

use crate::query_engine::QueryEngine;
use crate::{error, info, Result};

pub async fn query_database(tx: UnboundedSender<Bot>, parsed_sql_file: impl AsRef<str>) {
    let mut base_bots: Vec<BaseBot> = QueryEngine::default()
        .get_bots(parsed_sql_file.as_ref())
        .await
        .unwrap();
    // TODO: STREAM: We can stream these instead of iterating over them
    while let Some(base_bot) = base_bots.pop() {
        let filled_bot: Bot = Bot::from(base_bot);
        match filled_bot.status {
            bulk_runner_bots::BotStatus::Ready(ref status) => {
                info!("{:<12} - {:?}", "QUERY:: Ready bot", status);
                tx.send(filled_bot).unwrap();
            }
            bulk_runner_bots::BotStatus::NotReady(status) => {
                info!("{:<12} - {:?}", "QUERY:: Not ready bot", status);
                continue;
            }
        }
    }
    drop(tx);
}

pub async fn cli_dispatch(mut dispatch_bots: VecDeque<(Bot, String)>, total_bots: usize) {
    let sempahore = Arc::new(tokio::sync::Semaphore::new(total_bots));

    let (dispatched_tx, mut dispatched_rx) = tokio::sync::mpsc::unbounded_channel();

    let blocking_task_handles = dispatch_bots
        .make_contiguous()
        .iter_mut() // .into_iter()
        .map(|(bot, process_name)| {
            let sempahore = sempahore.clone();

            // Can I just pull a permit here?
            // then drop afte rthe spawn_blocking call?
            let bot = bot.clone();
            let process_name = process_name.clone();
            let dispatched_tx = dispatched_tx.clone();

            tokio::task::spawn_blocking(move || {
                let res = threaded_dispatch(&bot, &process_name, sempahore.clone());
                dispatched_tx.send(res).unwrap();
                drop(sempahore);
                drop(dispatched_tx);
            })
        })
        .collect::<Vec<_>>();

    let t1 = tokio::spawn(async move {
        for v in blocking_task_handles {
            v.await.unwrap();
        }
    });
    drop(dispatched_tx);

    let t2 = tokio::spawn(async move {
        while let Some(res) = dispatched_rx.recv().await {
            match res {
                Ok(_) => info!("->> {:<12} - {}", "DISPATCH:: OK", "Bot ran successfully!"),
                Err(e) => error!("->> {:<12} - {:?}", "DISPATCH:: ERROR", e),
            }
        }
        dispatched_rx.close();
        drop(dispatched_rx);
    });

    let _ = futures::future::join_all(vec![t1, t2]).await;
}

#[tokio::main]
async fn threaded_dispatch(
    bot: &Bot,
    process_name: &str,
    sempahore: Arc<tokio::sync::Semaphore>,
) -> Result<()> {
    info!("INNER:: Spawn local: {:}", &process_name);
    let permit = sempahore.acquire_owned().await.unwrap();
    // let process_name: &str = &process_name;
    let commander = crate::command_builder::AutomateBuilderBase::default()
        .with_sso()
        .with_process(process_name)
        .with_resource(&bot.name)
        .build();

    let res = bot.dispatch(commander.into()).await;
    tokio::task::yield_now().await;
    check_err(res).await;
    drop(permit);

    Ok(())
}

pub async fn check_err(res: bulk_runner_bots::Result<()>) {
    match res {
        Ok(_) => info!("->> {:<12} - {}", "CHECK_ERR:: OK", "Bot ran successfully!"),
        Err(e) => error!("->> {:<12} - {:?}", "CHECK_ERR:: ERROR", e),
    }
}

// From within the .map(|(bot, process_name)| {}) closure
// no longer used as we do this via spawn_blocking now
//
// When it was only async - no handling via threading
// let local = tokio::task::LocalSet::new();
// let mut js = tokio::task::JoinSet::new();
//
// When it was only async - no handling via threading
// let handle: JoinHandle<Result<()>> = local.spawn_local(async move {
//     match tokio::task::spawn_local(async move {
//         info!("INNER:: Spawn local: {:}", &process_name);
//         let permit = sempahore.acquire_owned().await.unwrap();
//         let process_name: &str = &process_name;
//         let commander = crate::command_builder::AutomateBuilderBase::default()
//             .with_sso()
//             .with_process(process_name)
//             .with_resource(&bot.name)
//             .build();
//
//         let res = bot.dispatch(commander.into()).await;
//         tokio::task::yield_now().await;
//         check_err(res).await;
//         drop(permit);
//     })
//     .await
//     {
//         Ok(_) => info!("->> {:<12} - {}", "DISPATCH:: OK", "Bot ran successfully!"),
//         Err(e) => error!("->> {:<12} - {:?}", "DISPATCH:: ERROR", e),
//     }
//
//     Ok(())
// });
// handle
//
//
//
// When it was only async - no handling via threading
// local
//     .run_until(async {
//         futures::future::join_all(handles).await;
//     })
//     .await;

// futures::future::join_all(handles).await;
