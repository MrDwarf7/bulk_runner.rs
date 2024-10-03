use deadpool_tiberius::tiberius::Row;
use tokio::process::{Child, Command};

use crate::query_controller::AutomateBuilderBase;
use crate::{
    prelude::{debug, error, info},
    Result, W,
};

#[derive(Copy, Clone, Debug)]
enum BotStatusReady {
    Idle,
    Pending,
}

#[derive(Copy, Clone, Debug)]
enum BotStatusNotReady {
    Offline,
    Unavailable,
    Private,
}

#[derive(Copy, Clone, Debug)]
enum BotStatus {
    Ready(BotStatusReady),
    NotReady(BotStatusNotReady),
}

impl From<BotStatus> for String {
    fn from(status: BotStatus) -> Self {
        match status {
            BotStatus::Ready(status) => match status {
                BotStatusReady::Idle => "IDLE".to_string().to_uppercase(),
                BotStatusReady::Pending => "PENDING".to_string(),
            },
            BotStatus::NotReady(status) => match status {
                BotStatusNotReady::Offline => "OFFLINE".to_string(),
                BotStatusNotReady::Private => "PRIVATE".to_string(),
                BotStatusNotReady::Unavailable => "UNAVAILABLE".to_string(),
            },
        }
    }
}

impl From<String> for BotStatus {
    fn from(status: String) -> Self {
        match status.to_uppercase().as_str() {
            "IDLE" => BotStatus::Ready(BotStatusReady::Idle),
            "PENDING" => BotStatus::Ready(BotStatusReady::Pending),
            "OFFLINE" => BotStatus::NotReady(BotStatusNotReady::Offline),
            "PRIVATE" => BotStatus::NotReady(BotStatusNotReady::Private),
            "UNAVAILABLE" => BotStatus::NotReady(BotStatusNotReady::Unavailable),
            _ => BotStatus::NotReady(BotStatusNotReady::Offline),
        }
    }
}

impl From<BotStatusReady> for BotStatus {
    fn from(status: BotStatusReady) -> Self {
        BotStatus::Ready(status)
    }
}

impl From<BotStatusNotReady> for BotStatus {
    fn from(status: BotStatusNotReady) -> Self {
        BotStatus::NotReady(status)
    }
}

impl From<BaseBot> for (String, String) {
    fn from(base_bot: BaseBot) -> Self {
        let name = base_bot.name.unwrap_or_default();
        let status = base_bot.status.unwrap_or_default();
        (name, status)
    }
}

#[derive(Clone, Debug)]
pub struct Bot {
    pub(crate) name: String,
    status: BotStatus,
}

impl Bot {
    pub async fn dispatch(&self, process_name: &str) -> Result<()> {
        // Do something with the bot
        // let mut cmd = Command::new()
        let (tx, rx) = tokio::sync::oneshot::channel();

        let cmd = Command::new(&*crate::prelude::DEFAULT_EXE_PATH);
        let commander = AutomateBuilderBase::default()
            .with_sso()
            .with_process(process_name)
            .with_resource(&self.name)
            .build();

        debug!(
            "->> {:<12} - {:?}",
            "DISPATCH:: Commander", &commander.args_vec
        );

        info!("->> {:<12}", "DISPATCH:: Commander");

        let handles = tokio::task::spawn(async move {
            let mut cmd = cmd;
            info!("->> {:<12} - {}", "DISPATCH:: Spawned", "child process");

            cmd.args(commander.args_vec);
            let future_output = cmd
                .spawn()
                .map_err(crate::error::Error::from)
                .expect("Failed to spawn");
            poll_child_process(future_output, tx).await
        });

        let name = self.name.clone();

        tokio::spawn(async move {
            match handles.await {
                Ok(_) => info!(
                    "->> {:<12} - {} - {:?}",
                    "DISPATCH:: OK", "Bot ran successfully!", &name
                ),

                Err(e) => error!(
                    "->> {:<12} - {}: {} - {:?}",
                    "DISPATCH:: ERR", "Bot failed to run! ", name, e
                ),
            };

            info!(
                "->> {:<12} - {}",
                "DISPATCH:: Child process ", "spawned successfully - waiting for output"
            );
            // info!("Child spawned successfully - waiting for output");

            let after = rx.await.unwrap();
            match after.status.success() {
                true => info!(
                    "->> {:<12} - {} - {:?}",
                    "DISPATCH:: Success & OK", "Bot ran successfully!", &name
                ),

                // info!("Bot: {} ran successfully", name),
                false => error!(
                    "->> {:<12} - {}: {} - {:?}",
                    "DISPATCH:: ERR & NotOk", "Bot failed to run! ", name, after
                ),
            }
        })
        .await?;

        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn if_available(&self) -> Option<Self> {
        match self.status {
            BotStatus::Ready(ready) => match ready {
                BotStatusReady::Idle => Some(self.clone()),
                BotStatusReady::Pending => None,
            },
            BotStatus::NotReady(not_ready) => match not_ready {
                BotStatusNotReady::Offline => None,
                BotStatusNotReady::Private => None,
                BotStatusNotReady::Unavailable => None,
            },
        }
    }
}

async fn poll_child_process(
    child: Child,
    tx_stop: tokio::sync::oneshot::Sender<std::process::Output>,
) {
    tokio::spawn(async move {
        let output = child
            .wait_with_output()
            .await
            .expect("Failed to wait on child");
        tx_stop.send(output).unwrap();
    })
    .await
    .unwrap_or_default();
}

#[derive(Default, Clone, Debug)]
pub struct BaseBot {
    pub(crate) name: Option<String>,
    pub(crate) status: Option<String>,
}

impl From<&mut Row> for BaseBot {
    fn from(row: &mut Row) -> Self {
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
            None => BotStatus::NotReady(BotStatusNotReady::Offline),
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
