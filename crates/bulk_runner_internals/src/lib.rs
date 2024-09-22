//! # Command Line
//!
//! This crate provides a simple interface for running `AutomateC` commands.
//!
//! # Example
//! ```
//! use internals::{AutomateCBuilder, AutomateC};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let automate = AutomateCBuilder::default()
//!     .with_sso()
//!     .with_process("AutomatedProcess")
//!     .with_resource("AutomatedWorker")
//!     .build();
//!
//! let output = automate.run().expect("Failed to run `AutomateC`");
//!
//! Ok(())
//! }
//! ```
//!
//! # Notes
//! - The `exe_path` is the path to the `AutomateC` executable.
//! - The `args` are the arguments to pass to `AutomateC`.
//! - The `args` are built using the `AutomateCBuilder` struct.
//!
//! # Async Example
//! ```
//! use internals::{AutomateCBuilder, AutomateC};    
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let automate = AutomateCBuilder::default()
//!     .with_sso()
//!     .with_process("AutomatedProcess")
//!     .with_resource("AutomatedWorker")
//!     .build();
//!
//! let output = automate.run_async().await.expect("Failed to run `AutomateC`");
//! println!("{:?}", output);
//!
//! Ok(())
//! }
//! ```

//! # Features
//! - `async` - Enable asynchronous execution of `AutomateC` commands.
//!
//! Access to this is done via the `AutomateC` struct and the [automatec::AutomateC::run_async] method.
//! This uses the `tokio` crate to execute the commands asynchronously.

mod automatec;
mod error;

//mod to_args;
//
// #[cfg(test)]
// mod tests;

#[cfg(feature = "async")]
mod automatec_futures;

pub use self::automatec::{AutomateC, AutomateCBuilder};
pub use self::error::Error;
//pub use self::to_args::*;

#[cfg(feature = "async")]
pub use self::automatec_futures::AutomateCFuture;

/// The result type for the `internals` crate.
/// This is a wrapper around the `anyhow::Result` type.
/// It is used to return a result from the `internals` crate.
pub type Result<T> = anyhow::Result<T, Error>;

use std::path::PathBuf;
use std::sync::LazyLock;

/// The default path to the `AutomateC` executable.
/// This is used as a fallback if the `exe_path` cannot be found.
///
/// WIP: At some point I may add the logic to search for the automatec.exe in the system PATH.
/// but for now this is used as a fallback stable default.
pub static DEFAULT_EXE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from("C:\\Program Files\\Blue Prism Limited\\Blue Prism Automate\\automatec.exe")
});
