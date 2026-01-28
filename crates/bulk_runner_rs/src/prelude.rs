// in-crate Error type
pub use std::path::PathBuf;
pub use std::sync::Arc;

pub use tracing::{debug, error, info, warn};

pub use crate::error::Error;

// in-crate result type
pub type Result<T> = std::result::Result<T, Error>;

// Wrapper struct
pub struct W<T>(pub T);

pub static DEFAULT_QUERY_FILE: &str = "bots.sql";

pub static PROD_HOST: &str = "PRDLGDB2";
pub static PROD_DB: &str = "BP_PRD";

pub use bulk_runner_bots::DEFAULT_EXE_PATH;
