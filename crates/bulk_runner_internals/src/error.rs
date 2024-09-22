//! # Error
//!
//! This module provides an error type for the `bulk_runner_internals` crate.
//!
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Generic error handler
    /// Mostly used for internal errors that are not related to the actual listener exe.
    #[error("Generic error handler: {0}")]
    Generic(String),

    /// AutomateC error handler
    /// In the case where the actual listener exe fails to execute a given command
    /// this error will be returned.
    #[error("AutomateC error: {0}")]
    AutomateC(String),
}
