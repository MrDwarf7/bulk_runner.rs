//! # AutomateC Futures
//!
//! This module provides a future for executing `AutomateC` commands asynchronously.
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
//! let future = automate.run_async();
//!
//! let output = future.await?;
//!
//! Ok(())
//! }
//! ```
//!

//! # Notes
//! - The `child` is the child process that is executing the `AutomateC` command.
/// - The `child` is an `Option<Child>` because it is not yet created until the `AutomateC` command is executed and resolved.
use crate::Error;
use futures::{Future, FutureExt};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::process::Child;

/// # AutomateCFuture
///
/// This struct represents a future that will be resolved when the `AutomateC` command is executed.
///
/// # Example
/// ```
/// use internals::{AutomateCBuilder, AutomateC};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let automate = AutomateCBuilder::default()
///     .with_sso()
///     .with_process("AutomatedProcess")
///     .with_resource("AutomatedWorker")
///     .build();
///
/// let future = automate.run_async();
///
/// let output = future.await?;
///
/// Ok(())
/// }
/// ```
///
/// # Notes
/// - The `child` is the child process that is executing the `AutomateC` command.
/// - The `child` is an `Option<Child>` because it is not yet created until the `AutomateC` command is executed and resolved.
#[derive(Debug)]
pub struct AutomateCFuture {
    /// The child process that is executing the `AutomateC` command.
    pub child: Option<Child>,
}

impl Future for AutomateCFuture {
    type Output = Result<std::process::Output, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self).poll_unpin(cx) {
            Poll::Ready(exit_status) => {
                let output = match exit_status {
                    Ok(output) => output,
                    Err(e) => return Poll::Ready(Err(Error::AutomateC(e.to_string()))),
                };
                Poll::Ready(Ok(output))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
