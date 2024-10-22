// in-crate Error type
pub use crate::error::Error;
pub use std::path::PathBuf;
use std::sync::LazyLock;

pub use tracing::{debug, error, info, warn};
// in-crate result type
pub type Result<T> = std::result::Result<T, Error>;

// Wrapper struct
pub struct W<T>(pub T);

pub static DEFAULT_QUERY_FILE: &str = "bots.sql";

pub static PROD_HOST: &str = "PRDLGDB2";
pub static PROD_DB: &str = "BP_PRD";

pub static DEFAULT_EXE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from("C:\\Program Files\\Blue Prism Limited\\Blue Prism Automate\\automatec.exe")
});

pub use std::sync::Arc;
