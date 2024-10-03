//! # `AutomateC`
//!
//! The `AutomateC` struct is used to run the `AutomateC` executable with a specific set of arguments.
//! This can be built using the `AutomateCBuilder` struct.
//!
//! # Example
//! ```
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use internals::{AutomateCBuilder, AutomateC};
//! let automate = internals::AutomateCBuilder::default()
//!     .with_sso()
//!     .with_process("AutomatedProcess")
//!     .with_resource("AutomatedWorker")
//!     .with_user("user")
//!     .with_password("password")
//!     .build();
//!
//! let output = automate.run().expect("Failed to run `AutomateC`");
//! Ok(())
//! }
//! ```
//!
//! # Notes
//! - The `exe_path` is the path to the `AutomateC` executable.
//! - The `args` are the arguments to pass to `AutomateC`.
//! - The `args` are built using the `AutomateCBuilder` struct.
//! - The `args` are built using the `AutomateCBuilder` struct.
//! - The `args` are built using the `AutomateCBuilder` struct.
//!
//! # Errors
//! - If the `AutomateC` executable is not found at the specified path, or is not a file, an error will be returned.
//! - If the `AutomateC` executable fails to execute, an error will be returned.
//!
//! # Returns
//! - The output of the `AutomateC` executable.
//!
//! # Panics
//! - If the `AutomateC` executable is not found at the specified path, or is not a file, the program will panic.
//! - If the `AutomateC` executable fails to execute, the program will panic.
//!

use crate::{Error, Result, DEFAULT_EXE_PATH};
use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display, hash::Hash, path::PathBuf};

#[cfg(feature = "async")]
use crate::AutomateCFuture;
#[cfg(feature = "async")]
use tokio::process::Command as TokioCommand;

/// The `AutomateC` struct represents a command to run `AutomateC` with a specific set of arguments.
///
/// This can be built using the `AutomateCBuilder` struct.
///
/// # Example
/// ```
/// use internals::{AutomateCBuilder, AutomateC};
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let automate = AutomateCBuilder::default()
///     .with_sso()
///     .with_process("AutomatedProcess")
///     .with_resource("AutomatedWorker")
///     .with_user("user")
///     .with_password("password")
///     .build();
///
/// let output = automate.run().expect("Failed to run `AutomateC`");
/// Ok(())
/// }
/// ```
///
/// # Notes
/// - The `exe_path` is the path to the `AutomateC` executable.
/// - The `args` are the arguments to pass to `AutomateC`.
/// - The `args` are built using the `AutomateCBuilder` struct.
/// - The `args` are built using the `AutomateCBuilder` struct.
#[derive(Debug, Clone, Default)]
pub struct AutomateC {
    /// The path to the `AutomateC` executable.
    pub exe_path: PathBuf,
    args: Vec<String>,
}

impl AutomateC {
    /// Runs the `AutomateC` command and returns the output.
    ///
    /// # Example
    /// ```
    /// use internals::{AutomateCBuilder, AutomateC};
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let automate = AutomateCBuilder::default()
    ///     .with_sso()
    ///     .with_process("AutomatedProcess")
    ///     .with_resource("AutomatedWorker")
    ///     .with_user("user")
    ///     .with_password("password")
    ///     .build();
    ///
    /// let output = automate.run().expect("Failed to run `AutomateC`");
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    /// - The `exe_path` is the path to the `AutomateC` executable.
    /// - The `args` are the arguments to pass to `AutomateC`.
    /// - The `args` are built using the `AutomateCBuilder` struct.
    /// - The `args` are built using the `AutomateCBuilder` struct.
    ///
    /// # Errors
    /// - If the `AutomateC` executable is not found at the specified path, or is not a file, an error will be returned.
    /// - If the `AutomateC` executable fails to execute, an error will be returned.
    ///
    /// # Returns
    /// - The output of the `AutomateC` executable.
    ///
    /// # Panics
    /// - If the `AutomateC` executable is not found at the specified path, or is not a file, the program will panic.
    /// - If the `AutomateC` executable fails to execute, the program will panic.
    #[cfg(not(feature = "async"))]
    pub fn run(&self) -> Result<std::process::Output> {
        let mut cmd = std::process::Command::new(&self.exe_path);
        cmd.args(&self.args);
        cmd.output().map_err(|e| Error::AutomateC(e.to_string()))
    }

    /// The `AutomateC` struct represents a command to run `AutomateC` with a specific set of arguments.
    /// This can be built using the `AutomateCBuilder` struct.
    ///
    /// This is the async version of the `AutomateC` struct.
    ///
    /// # Example
    /// ```
    /// use internals::{AutomateCBuilder, AutomateC};
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let automate = AutomateCBuilder::default()
    ///     .with_sso()
    ///     .with_process("AutomatedProcess")
    ///     .with_resource("AutomatedWorker")
    ///     .with_user("user")
    ///     .with_password("password")
    ///     .build();
    ///
    /// let output = automate.run_async().await?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Notes
    /// - The `exe_path` is the path to the `AutomateC` executable.
    /// - The `args` are the arguments to pass to `AutomateC`.
    /// - The `args` are built using the `AutomateCBuilder` struct.
    /// - The `args` are built using the `AutomateCBuilder` struct.
    ///
    /// # Errors
    /// - If the `AutomateC` executable is not found at the specified path, or is not a file, an error will be returned.
    /// - If the `AutomateC` executable fails to execute, an error will be returned.
    ///
    /// # Returns
    /// - The output of the `AutomateC` executable.
    ///
    /// # Panics
    /// - If the `AutomateC` executable is not found at the specified path, or is not a file, the program will panic.
    /// - If the `AutomateC` executable fails to execute, the program will panic.
    ///
    #[allow(clippy::unused_async)]
    #[cfg(feature = "async")]
    pub async fn run(&self) -> Result<AutomateCFuture> {
        let mut cmd = TokioCommand::new(&self.exe_path);
        cmd.args(&self.args);
        let child = cmd.spawn().map_err(|e| Error::AutomateC(e.to_string()))?;
        Ok(AutomateCFuture { child: Some(child) })
    }

    /// Provides access to the async version even if the feature is not enabled.
    #[allow(clippy::unused_async)]
    pub async fn run_async(&self) -> Result<AutomateCFuture> {
        #[cfg(feature = "async")]
        {
            self.run().await
        }
        #[cfg(not(feature = "async"))]
        {
            self.run()
        }
    }
}

/// The `AutomateCBuilder` struct is used to build an `AutomateC` struct.
/// This is the recommended way to create an `AutomateC` struct.
///
/// # Example
/// ```
/// use internals::{AutomateCBuilder, AutomateC};
/// let automate = AutomateCBuilder::default()
///     .with_sso()
///     .with_process("AutomatedProcess")
///     .with_resource("AutomatedWorker")
///     .with_user("user")
///     .with_password("password")
///     .build();
/// ```
///
/// # Notes
/// - The `exe_path` is the path to the `AutomateC` executable.
/// - The `args` are the arguments to pass to `AutomateC`.
/// - The `args` are built using the `AutomateCBuilder` struct.
/// - The `args` are built using the `AutomateCBuilder` struct.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Hash)]
pub struct AutomateCBuilder {
    pub exe_path: Option<PathBuf>,
    args: Vec<String>,
}

impl Default for AutomateCBuilder {
    /// Creates a new `AutomateCBuilder` with the default path to the `AutomateC` executable.
    fn default() -> Self {
        Self::new(Some(handle_path(DEFAULT_EXE_PATH.clone()))).unwrap_or_else(|e| {
            println!("Failed to create AutomateCBuilder with default path: {e}");
            std::process::exit(1);
        })
    }
}

impl AutomateCBuilder {
    /// Creates a new `AutomateCBuilder` with the specified path to the `AutomateC` executable.
    /// Use `default()` if you want to use the default path to the `AutomateC` executable.
    ///
    /// # Example
    /// ```
    /// use internals::AutomateCBuilder;
    /// use std::path::PathBuf;
    /// let automate = AutomateCBuilder::new(Some(PathBuf::from("C:\\Program Files\\Blue Prism Limited\\Blue Prism Automate\\automatec.exe"))).unwrap();
    /// ```
    ///
    /// # Errors
    /// - If the `AutomateC` executable is not found at the specified path, or is not a file, an error will be returned.
    ///
    /// # Panics
    /// - If the `AutomateC` executable is not found at the specified path, or is not a file, the program will panic.
    pub fn new(exe_path: Option<PathBuf>) -> anyhow::Result<AutomateCBuilder> {
        if let Some(path) = exe_path {
            if !path.exists() {
                bail!(
                    "`AutomateC` executable not found at path: {}",
                    path.display()
                )
            }
            if !path.is_file() {
                bail!("`AutomateC` executable is not a file: {}", path.display())
            }
            Ok(AutomateCBuilder {
                exe_path: Some(path),
                args: vec![],
            })
        } else {
            Ok(AutomateCBuilder {
                exe_path: Some(DEFAULT_EXE_PATH.clone()),
                args: vec![],
            })
        }
    }
}

impl AutomateCBuilder {
    /// Internal function to add the /sso argument to the args.
    /// This is used for calling the `AutomateC` executable with the /sso flag, the public method is `with_sso`.
    fn sso(&mut self) -> &mut Self {
        self.args.push("/sso".to_string());
        self
    }

    /// Internal function to add the /run argument to the args.
    /// This is used for calling the `AutomateC` executable with the /run flag, the public method is `with_process`.
    fn run(&mut self) -> &mut Self {
        self.args.push("/run".to_string());
        self
    }

    /// Internal function to add the /resource argument to the args.
    /// This is used for calling the `AutomateC` executable with the /resource flag, the public method is `with_resource`.
    fn resource(&mut self) -> &mut Self {
        self.args.push("/resource".to_string());
        self
    }

    /// Internal function to add the /user argument to the args.
    /// This is used for calling the `AutomateC` executable with the /user flag, the public method is `with_user`.
    fn user(&mut self) -> &mut Self {
        self.args.push("/user".to_string());
        self
    }

    /// Internal function to add the /password argument to the args.
    /// This is used for calling the `AutomateC` executable with the /password flag, the public method is `with_password`.
    fn password(&mut self) -> &mut Self {
        self.args.push("/password".to_string());
        self
    }
}

impl AutomateCBuilder {
    /// Adds the /sso argument to the args.
    ///
    /// When this is called, the equivalent command would be:
    /// automatec /sso
    pub fn with_sso(&mut self) -> &mut Self {
        self.sso();
        self
    }

    /// Adds the /run argument to the args and the process name to the args.
    ///
    /// When this is called, the equivalent command would be:
    /// automatec /run `"ProcessName"`
    pub fn with_process(&mut self, process: &str) -> &mut Self {
        self.run();
        self.args.push(process.to_string());
        self
    }

    /// Adds the /resource argument to the args and the resource name to the args.
    /// If the resource name contains spaces, it will be enclosed in quotes.
    ///
    /// When this is called, the equivalent command would be:
    /// automatec /resource `"ResourceName"`
    pub fn with_resource(&mut self, resource: &str) -> &mut Self {
        self.resource();
        self.args.push(resource.to_string());
        self
    }

    /// Adds the /user argument to the args and the user name to the args.
    /// If the user name contains spaces, it will be enclosed in quotes.
    ///
    /// When this is called, the equivalent command would be:
    /// automatec /user `"UserName"`
    pub fn with_user(&mut self, user: &str) -> &mut Self {
        self.user();
        let user = handle_quotes(user);
        self.args.push(user.to_string());
        self
    }

    /// Adds the /password argument to the args and the password to the args.
    /// If the password contains spaces, it will be enclosed in quotes.
    ///
    /// When this is called, the equivalent command would be:
    /// automatec /password "Password"
    pub fn with_password(&mut self, password: &str) -> &mut Self {
        self.password();
        let password = handle_quotes(password);
        self.args.push(password.to_string());
        self
    }
}

// fn build_args(args: Vec<String>) -> Vec<String> {
//     let mut arg_values = vec![];
//
//     for arg in args {
//         arg_values.push(arg);
//     }
//     arg_values
// }

/// Internal function to handle quotes in the arguments.
/// This is used for calling the `AutomateC` executable with the /user and /password flags, the public method is `with_user` and `with_password`.
fn handle_quotes(arg: &str) -> Cow<'_, str> {
    if arg.contains(' ') || arg.contains('"') {
        let arg = format!("\"{arg}\"");
        Cow::Owned(arg)
    } else {
        Cow::Borrowed(arg)
    }
}

/// Internal function to handle the path to the `AutomateC` executable.
fn handle_path(path: PathBuf) -> PathBuf {
    let known = &DEFAULT_EXE_PATH;

    if path.exists() && path.is_file() {
        path
    } else {
        known.to_path_buf()
    }
}

impl AutomateCBuilder {
    /// Builds the `AutomateC` struct.
    ///
    /// # Example
    /// ```
    /// use internals::{AutomateCBuilder, AutomateC};
    /// use std::path::PathBuf;
    /// let automate = AutomateCBuilder::new(Some(PathBuf::from("C:\\Program Files\\Blue Prism Limited\\Blue Prism Automate\\automatec.exe")))
    ///     .expect("Failed to create AutomateCBuilder")
    ///     .with_sso()
    ///     .with_process("AutomatedProcess")
    ///     .with_resource("AutomatedWorker")
    ///     .with_user("user")
    ///     .with_password("password")
    ///     .build();
    /// ```
    ///
    /// # Notes
    /// - The `exe_path` is the path to the `AutomateC` executable.
    /// - The `args` are the arguments to pass to `AutomateC`.
    /// - The `args` are built using the `AutomateCBuilder` struct.
    /// - The `args` are built using the `AutomateCBuilder` struct.
    pub fn build(&self) -> AutomateC {
        AutomateC {
            exe_path: self.exe_path.clone().unwrap_or(DEFAULT_EXE_PATH.clone()),
            args: self.args.clone(),
        }
    }
}

impl Display for AutomateC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Display for AutomateCBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
