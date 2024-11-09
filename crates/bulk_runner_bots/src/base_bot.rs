use std::fmt::Display;
use std::future::IntoFuture;
use std::process::Output;

use deadpool_tiberius::tiberius::Row;
use tokio::process::{Child, Command};

use crate::bot_types::{BotStatus, BotStatusNotReady, BotStatusReady};
use crate::{debug, error, info, Result, W};
// use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Bot {
    pub name: String,
    pub status: BotStatus,
}

impl Bot {
    pub async fn dispatch(&self, commander: Vec<String>) -> Result<()> {
        let (tx_stop, rx) = tokio::sync::oneshot::channel();

        let cmd = Command::new(&*crate::DEFAULT_EXE_PATH);

        // NOTE: We now hand through the proces name & the bot name ✔
        let handle = spawn_child_proc(tx_stop, cmd, commander).await;
        check_handle(handle, &self.name).await;

        // check_handle(handles, &name).await;
        #[rustfmt::skip]
        info!("{:<12} - {}", "DISPATCH:: Child process ", "spawned successfully - waiting for output");

        let after = rx.await.unwrap();
        check_status(after, &self.name);

        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_logged_out(&self) -> bool {
        match &self.status {
            BotStatus::Ready(ready) => matches!(ready, BotStatusReady::LoggedOut),
            BotStatus::NotReady(_) => false,
        }
    }

    pub fn is_available(&self) -> Option<Bot> {
        match &self.status {
            BotStatus::Ready(ready) => match ready {
                BotStatusReady::Idle => Some(self.clone()),
                BotStatusReady::Pending => None,
                BotStatusReady::LoggedOut => Some(self.clone()),
            },
            BotStatus::NotReady(not_ready) => match not_ready {
                BotStatusNotReady::Offline => None,
                BotStatusNotReady::Private => None,
                BotStatusNotReady::Unavailable => None,
            },
        }
    }
}

fn check_status(output: Output, name: impl AsRef<str> + Display) {
    match output.status.success() {
        true => info!(
            "->> {:<12} - {} - {name}",
            "DISPATCH:: OK", "Bot ran successfully!"
        ),

        false => error!(
            "->> {:<12} - {}: {name} - {output:?}",
            "DISPATCH:: ERR", "Bot failed to run! "
        ),
    };
}

async fn spawn_child_proc(
    tx_stop: tokio::sync::oneshot::Sender<std::process::Output>,
    mut cmd: Command,
    commander: Vec<String>,
) -> tokio::task::JoinHandle<()> {
    // let cmd = Command::new(&*crate::prelude::DEFAULT_EXE_PATH);
    // let commander = bulk_runner_query::AutomateBuilderBase::default()
    //     .with_sso()
    //     .with_process(process_name)
    //     .with_resource(&name)
    //     .build();

    #[rustfmt::skip]
    debug!("->> {:<12} - {:?}", "DISPATCH:: Commander", &commander);
    info!("{:<12}", "DISPATCH:: Child proc");

    //     let handles =
    // check_handle(
    let handle = tokio::spawn(async move {
        // let mut cmd = cmd;
        info!("->> {:<12} - {}", "DISPATCH:: Spawned", "child process");

        cmd.args(commander);
        let future_output = cmd
            .spawn()
            .map_err(crate::error::Error::from)
            .expect("Failed to spawn");
        let h = wait_on_child_proc(future_output, tx_stop);
        tokio::task::yield_now().await; // Yield because there's nothing else to do while automatec.exe runs
        h.await;
    });

    // &name,
    // )
    // .await;
    // check_handle(handles, &name).await;

    handle.into_future()
}

async fn wait_on_child_proc(
    child: Child,
    tx_stop: tokio::sync::oneshot::Sender<std::process::Output>,
) {
    tokio::spawn(async move {
        let output = child
            .wait_with_output()
            .await
            .expect("Failed to wait on child");
        tokio::task::yield_now().await;
        tx_stop.send(output).unwrap();
    });
    // We don't have to await right?
    //     .await
    //     .unwrap_or_default();
}

async fn check_handle(handle: tokio::task::JoinHandle<()>, name: impl AsRef<str> + Display) {
    match handle.await {
        Ok(_) => info!(
            "->> {:<12} - {} - {}",
            "DISPATCH:: OK", "Bot ran successfully!", &name
        ),

        Err(e) => error!(
            "->> {:<12} - {}: {name} - {e}",
            "DISPATCH:: ERR", "Bot failed to run! "
        ),
    };
}

#[derive(Default, Clone, Debug)]
pub struct BaseBot {
    pub(crate) name: Option<String>,
    pub(crate) status: Option<String>,
}

impl From<&Row> for BaseBot {
    fn from(row: &Row) -> Self {
        let name: String = row
            .try_get(0)
            .unwrap_or(Some("FailedToGet"))
            .unwrap_or_default()
            .to_string();

        let status = row
            .try_get(1)
            .unwrap_or(Some("FailedToGet"))
            .unwrap_or_default()
            .to_string();

        BaseBot {
            name: Some(name),
            status: Some(status),
        }
    }
}

impl From<BaseBot> for Bot {
    fn from(base_bot: BaseBot) -> Self {
        let name = match base_bot.name {
            Some(name) => name.to_uppercase(),
            None => String::default(),
        };

        let status: BotStatus = match base_bot.status {
            Some(status) => status.into(),
            None => BotStatus::NotReady(BotStatusNotReady::Unavailable),
        };

        Bot { name, status }
    }
}

impl From<&BaseBot> for Bot {
    fn from(value: &BaseBot) -> Self {
        let name = value.name.clone().unwrap_or_default();
        let status = value.status.clone().unwrap_or_default();
        Bot {
            name: name.to_uppercase(),
            status: status.into(),
        }
    }
}

impl W<Vec<Bot>> {
    pub fn new(bots: Vec<Bot>) -> Self {
        W(bots)
    }
}

// FIX: Pretty sure this has the potential to hang indefinitely. If we poll free bots - and the supplied query doesn't cull out
// bots that are occupied, we'll just hang here forever.
impl futures::Stream for Bot {
    type Item = Bot;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match self.status {
            BotStatus::Ready(_) => std::task::Poll::Ready(Some(self.clone())),
            BotStatus::NotReady(_) => std::task::Poll::Pending,
        }
    }
}
