use deadpool_tiberius::tiberius::Row;

use crate::bot_types::{BotStatus, BotStatusNotReady, BotStatusReady};

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Bot {
    pub name:   String,
    pub status: BotStatus,
}

impl Bot {
    #[must_use]
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    #[inline]
    pub fn is_logged_out(&self) -> bool {
        match &self.status {
            BotStatus::Ready(ready) => matches!(ready, BotStatusReady::LoggedOut),
            BotStatus::NotReady(_) => false,
        }
    }

    #[must_use]
    #[inline]
    pub fn is_available(&self) -> Option<Bot> {
        match &self.status {
            BotStatus::Ready(ready) => {
                match ready {
                    BotStatusReady::Pending => None,
                    BotStatusReady::Idle | BotStatusReady::LoggedOut => Some(self.clone()),
                }
            }
            BotStatus::NotReady(not_ready) => {
                match not_ready {
                    BotStatusNotReady::Offline
                    | BotStatusNotReady::Private
                    | BotStatusNotReady::Unavailable => None,
                }
            }
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct BaseBot {
    pub(crate) name:   Option<String>,
    pub(crate) status: Option<String>,
}

impl From<&Row> for BaseBot {
    #[inline]
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
            name:   Some(name),
            status: Some(status),
        }
    }
}

impl From<BaseBot> for Bot {
    #[inline]
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
    #[inline]
    fn from(value: &BaseBot) -> Self {
        let name = value.name.clone().unwrap_or_default();
        let status = value.status.clone().unwrap_or_default();
        Bot {
            name:   name.to_uppercase(),
            status: status.into(),
        }
    }
}

impl futures::Stream for Bot {
    type Item = Bot;

    #[inline]
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
