use std::fmt::Display;
use std::process::Output;

use deadpool_tiberius::tiberius::Row;
use tokio::process::Command;

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
        let (tx_stop, rx_stop) = tokio::sync::oneshot::channel();

        let cmd = Command::new(&*crate::DEFAULT_EXE_PATH);

        spawn_child_proc(tx_stop, cmd, commander).await;

        #[rustfmt::skip]
        info!("{:<12} - {}", "DISPATCH:: Child process ", "spawned successfully - waiting for output");

        let after = rx_stop.await.unwrap();
        CheckStatus::from(after).check_status(&self.name);

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

enum CheckStatus {
    Success(Output),
    Fail(Output),
}

impl From<Output> for CheckStatus {
    fn from(output: Output) -> Self {
        match output.status.success() {
            true => CheckStatus::Success(output),
            false => CheckStatus::Fail(output),
        }
    }
}

impl CheckStatus {
    fn check_status(&self, name: impl AsRef<str> + Display) {
        match self {
            CheckStatus::Success(output) => info!(
                "->> {:<12} - {}: {name} - with output: {}",
                "DISPATCH:: OK", "Job is now running on", output.status
            ),
            CheckStatus::Fail(output) => error!(
                "->> {:<12} - {}: {name} - {output:?}",
                "DISPATCH:: ERR", "Job has failed to start running on"
            ),
        };
    }
}

async fn spawn_child_proc(
    tx_stop: tokio::sync::oneshot::Sender<std::process::Output>,
    mut cmd: Command,
    commander: Vec<String>,
) {
    #[rustfmt::skip]
    debug!("->> {:<12} - {:?}", "DISPATCH:: Commander", &commander);
    info!("{:<12}", "DISPATCH:: Child proc");

    let cmd_for_print = commander.clone();

    match tokio::spawn(async move {
        let output = tokio::task::spawn_blocking(move || {
            info!("->> {:<12} - {}", "DISPATCH:: Spawned", "Child Proc");
            cmd.args(commander.clone())
                .spawn()
                .map_err(crate::error::Error::from)
                .expect("Failed to spawn")
                .wait_with_output()
        })
        .await
        .unwrap()
        .await
        .expect("Failed to wait on child");

        tx_stop.send(output).unwrap();
    })
    .await
    {
        Ok(_) => info!("->> {:<12} - {}", "DISPATCH:: OK", "Task dispatched!"),
        Err(e) => error!(
            "->> {:<12} - Tried to run {cmd_for_print:?} - {e}",
            "DISPATCH:: ERR",
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
