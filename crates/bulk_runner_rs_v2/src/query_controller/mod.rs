mod command_builder;
mod db_info;
mod query_engine;

use db_info::DbInfo;

use dashmap::rayon::map::OwningIter;
use dashmap::DashMap as HashMap;
use rayon::prelude::*;

use rayon::prelude::IntoParallelIterator;
use tokio::sync::mpsc::UnboundedSender;

pub use self::command_builder::AutomateBuilderBase;
use self::query_engine::QueryEngine;
use crate::prelude::*;
use crate::{BaseBot, Bot};

pub async fn query_database(tx: UnboundedSender<Bot>, parsed_sql_file: &str) {
    let mut base_bots: Vec<BaseBot> = QueryEngine::default()
        .get_bots(parsed_sql_file)
        .await
        .unwrap();

    info!("->> {:<12} - {:?}", "QUERY:: base_bots", base_bots);

    while let Some(base_bot) = base_bots.pop() {
        let filled_bot: Bot = Bot::from(base_bot);
        info!("->> {:<12} - {:?}", "QUERY:: Filled bot", filled_bot);

        tx.send(filled_bot).unwrap();
    }
    drop(tx);
}

pub async fn cli_dispatch(dispatch_bots: HashMap<String, Bot>, total_bots: usize) -> Result<()> {
    let sempahore = Arc::new(tokio::sync::Semaphore::new(total_bots));
    let dispatch_bots: OwningIter<String, Bot> = dispatch_bots.into_par_iter();

    // let future_holder = Arc::new(FutureHolder::default());

    // in tokio here
    let handles = dispatch_bots
        // .into_par_iter()
        .map(|(process_name, bot)| {
            // ------- no tokio here??
            let sempahore = sempahore.clone();

            // let mut future_holder = Arc::clone(&future_holder);
            // let rt = tokio::runtime::Builder::new_multi_thread() // BUG: single thread instead?
            //     .worker_threads(2)
            //     .enable_all()
            //     .build()
            //     .unwrap();
            // BUG: Thread local?? maybe instead?

            let local = tokio::task::LocalSet::new();

            // BUG: We are dropping a join handle or something here - Need to see why that is
             
            local.spawn_local(async move {
                tokio::task::spawn_local(async move {
                    let permit = sempahore.acquire_owned().await.unwrap();
                    let res = bot.dispatch(&process_name).await;
                    if let Err(e) = res {
                        #[rustfmt::skip]
                        error!( "->> {:<12} - Error running bot: {:#?} with process: {}", "DISPATCH:: ERROR ", bot, process_name);
                        error!("->> {:<12} - {:#?}", "DISPATCH:: ERROR ", e);
                    }
                    drop(permit);
                    tokio::task::yield_now().await;
                })
                .await
                .unwrap();
            })
            // runtime.block_on(async {
            //     let mut local_thread = local_thread;
            //     local.spawn_local(async move {
            // (&mut local_thread).await;
            //         // local_thread.await;
            //     }).await.unwrap();
            // })
            //
        })
        .collect::<Vec<_>>();

    futures::future::join_all(handles).await;


    Ok(())
}


/////////// Renditions of the threading section that cause bugs or aren't logically correct

// tokio::spawn(async move {
//     let permit = sempahore.acquire_owned().await.unwrap();
//     let res = bot.dispatch(&process_name).await;
//     if let Err(e) = res {
//         #[rustfmt::skip]
//         error!("->> {:<12} - Error running bot: {:#?} with process: {}", "DISPATCH:: ERROR ", bot, process_name);
//         error!("->> {:<12} - {:#?}", "DISPATCH:: ERROR ", e);
//     }
//     drop(permit);
// })
// })

// let runtime_handle = runtime.block_on(async {
//     tokio::spawn(async move {
//         let permit = sempahore.acquire_owned().await.unwrap();
//         let res = bot.dispatch(&process_name).await;
//         if let Err(e) = res {
//             #[rustfmt::skip]
//             error!("->> {:<12} - Error running bot: {:#?} with process: {}", "DISPATCH:: ERROR ", bot, process_name);
//             error!("->> {:<12} - {:#?}", "DISPATCH:: ERROR ", e);
//         }
//         drop(permit);
//     })
// });
// tokio::pin!(runtime_handle);
// runtime_handle

// local_future
// .collect::<Vec<_>>());

// futures::future::join_all(j_handles).await;

// for (process_name, bot) in dispatch_bots.par_iter() {
//     let permit = sempahore.clone().acquire_owned().await?;
//     // let dispatch_context = clonable_dispatch_context.clone();
//     let handle = tokio::spawn(async move {
//         // Do something with the bot
//         // &*dispatch_context
//         let res = bot.dispatch(&process_name).await;
//         if let Err(e) = res {
//             #[rustfmt::skip]
//             error!("Error running bot: {} with process: {}", bot.name, process_name);
//             error!("Error: {}", e);
//         }
//         drop(permit);
//     });
//     handles.push(handle);
// }

///////////////// 



// struct FutureHolder<T> {
//     inner_future: Pin<Box<dyn std::future::Future<Output = T> + Send + Sync>>,
// }
//
// impl Default for FutureHolder<()> {
//     fn default() -> Self {
//         Self {
//             inner_future: Box::pin(async {}),
//         }
//     }
// }
//
// impl<T> FutureHolder<T> {
//     pub fn new<F>(f: F) -> Self
//     where
//         F: std::future::Future<Output = T> + Send + Sync + 'static,
//     {
//         Self {
//             inner_future: Box::pin(f),
//         }
//     }
//
//     pub fn assign_inner<F>(&mut self, f: F)
//     where
//         F: std::future::Future<Output = T> + Send + Sync + 'static,
//     {
//         self.inner_future = Box::pin(f);
//     }
//
//     pub fn get_inner(&self) -> &Pin<Box<dyn std::future::Future<Output = T> + Send + Sync>> {
//         &self.inner_future
//     }
//
// }
//
// impl<T> std::future::Future for FutureHolder<T> {
//     type Output = T;
//
//     fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
//         self.inner_future.as_mut().poll(cx)
//     }
// }
//
//
// impl FutureHolder<()> {
//     pub fn resolve(&mut self) {
//         let poll = self.inner_future.as_mut().poll(&mut std::task::Context::from_waker(futures::task::noop_waker_ref()));
//         while poll.is_pending() {
//             std::thread::yield_now();
//         }
//     }
//
//     pub async fn resolve_async(&mut self) {
//         let poll = self.inner_future.poll_unpin(&mut std::task::Context::from_waker(futures::task::noop_waker_ref()));
//         while poll.is_pending() {
//             tokio::task::yield_now().await;
//         }
//
//     }
// }
//
// // unsafe impl Sync for FutureHolder<()> {}
//
