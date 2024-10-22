use std::collections::VecDeque;
use std::sync::Arc;

use tokio::task::JoinHandle;

use crate::{error, info, Result};

use crate::query_engine::QueryEngine;
// use crate::prelude::*;
use bulk_runner_bots::{BaseBot, Bot};
use tokio::sync::mpsc::UnboundedSender;

// use dashmap::DashMap as HashMap;

pub async fn query_database(tx: UnboundedSender<Bot>, parsed_sql_file: impl AsRef<str>) {
    let mut base_bots: Vec<BaseBot> = QueryEngine::default()
        .get_bots(parsed_sql_file)
        .await
        .unwrap();

    info!("->> {:<12} - {:?}", "QUERY:: base_bots", base_bots);

    // TODO: STREAM: We can stream these instead of iterating over them
    while let Some(base_bot) = base_bots.pop() {
        let filled_bot: Bot = Bot::from(base_bot);
        info!("->> {:<12} - {:?}", "QUERY:: Filled bot", filled_bot);

        tx.send(filled_bot).unwrap();
    }
    drop(tx);
}

pub async fn cli_dispatch(
    mut dispatch_bots: VecDeque<(Bot, String)>,
    //HashMap<Bot, String>,
    //Vec<(Bot, String)>,
    total_bots: usize,
) {
    let sempahore = Arc::new(tokio::sync::Semaphore::new(total_bots));
    // let dispatch_bots = dispatch_bots.into_iter().collect::<Vec<_>>(); /// For just hashmap, vec version we can just start main body

    let local = tokio::task::LocalSet::new();

    let handles = dispatch_bots
        .make_contiguous()
        .iter_mut() // .into_iter()
        .map(|(bot, process_name)| {
            let sempahore = sempahore.clone();
            let bot = bot.clone();
            let process_name = process_name.clone();

            let handle: JoinHandle<Result<()>> = local.spawn_local(async move {
                // warn!("OUTER:: Spawn local called: {:?}", process_name);

                match tokio::task::spawn_local(async move {
                    info!("INNER:: Spawn local called for: {:}", &process_name);
                    let permit = sempahore.acquire_owned().await.unwrap();
                    let process_name: &str = &process_name;
                    let commander = crate::command_builder::AutomateBuilderBase::default()
                        .with_sso()
                        .with_process(&process_name)
                        .with_resource(&bot.name)
                        .build();

                    let res = bot.dispatch(commander.into()).await;
                    check_err(res).await;
                    drop(permit);
                    tokio::task::yield_now().await;
                })
                .await
                {
                    Ok(_) => info!("->> {:<12} - {}", "DISPATCH:: OK", "Bot ran successfully!"),
                    Err(e) => error!("->> {:<12} - {:?}", "DISPATCH:: ERROR", e),
                }

                Ok(())
            });
            handle
        })
        .collect::<Vec<_>>();

    local
        .run_until(async {
            futures::future::join_all(handles).await;
        })
        .await;

    // futures::future::join_all(handles).await;
}

pub async fn check_err(res: bulk_runner_bots::Result<()>) {
    match res {
        Ok(_) => info!("->> {:<12} - {}", "CHECK_ERR:: OK", "Bot ran successfully!"),
        Err(e) => error!("->> {:<12} - {:?}", "CHECK_ERR:: ERROR", e),
    }
}
