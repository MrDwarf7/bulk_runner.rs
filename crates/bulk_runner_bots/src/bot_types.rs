use crate::base_bot::BaseBot;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum BotStatusReady {
    Idle,
    Pending,
    LoggedOut,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum BotStatusNotReady {
    Offline,
    Unavailable,
    Private,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum BotStatus {
    Ready(BotStatusReady),
    NotReady(BotStatusNotReady),
}

impl From<BotStatus> for String {
    fn from(status: BotStatus) -> Self {
        match status {
            BotStatus::Ready(status) => {
                match status {
                    BotStatusReady::Idle => "IDLE".to_string().to_uppercase(),
                    BotStatusReady::Pending => "PENDING".to_string(),
                    BotStatusReady::LoggedOut => "LOGGED OUT".to_string(),
                }
            }
            BotStatus::NotReady(status) => {
                match status {
                    BotStatusNotReady::Offline => "OFFLINE".to_string(),
                    BotStatusNotReady::Private => "PRIVATE".to_string(),
                    BotStatusNotReady::Unavailable => "UNAVAILABLE".to_string(),
                }
            }
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
            "LOGGED OUT" => BotStatus::Ready(BotStatusReady::LoggedOut),
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
