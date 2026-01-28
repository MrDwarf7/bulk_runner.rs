mod base_bot;
mod base_bot_dispatch;
mod bot_output;
mod bot_types;
mod error;

use std::path::PathBuf;
use std::sync::LazyLock;

use tracing::{debug, error, info};

pub use crate::base_bot::{BaseBot, Bot};
pub use crate::base_bot_dispatch::dispatch;
#[allow(unused_imports)]
pub use crate::bot_output::BotOutput;
pub use crate::bot_types::{BotStatus, BotStatusNotReady, BotStatusReady};
pub use crate::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub struct W<T>(pub T);

// Need Result & Error types

pub static DEFAULT_EXE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from("C:\\Program Files\\Blue Prism Limited\\Blue Prism Automate\\automatec.exe")
});
