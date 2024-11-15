use std::str::FromStr;

use crate::Result;
use clap::{command, Parser, ValueEnum};

use crate::prelude::*;

#[derive(Parser, Debug)]
#[command(
    name = "BulkRunner.rs",
    about = "A CLI tool to run Blue Prism processes (Via AutomateC dispatch) in bulk.",
    author = "Blake B.",
    long_about = "\n
    BulkRunner.rs is a CLI tool designed to facilitate the execution of Blue Prism AutomateC processes in bulk.
    It streamlines the process of launching multiple instances/process via the Control Room, each targeting a distinct resource or bot.
    This is particularly useful during change over periods where multiple bots need to be transitioned from one process to another.",
    version = std::env!("CARGO_PKG_VERSION"),
    arg_required_else_help = true,
    styles=get_styles()
)]
pub struct Cli {
    /// The process to run on all the bots pulled by the SQL query.
    #[arg(index = 1, help = "The process to run the bots on.", value_hint = clap::ValueHint::Other)]
    pub process: String,

    /// The number of bots to run concurrently.
    /// Limits the stress-load on the machine running the cli
    #[arg(short = 'c', long = "concurrency_limit", default_value = "30", value_hint = clap::ValueHint::Other, long_help = "The number of bots to run concurrently. Limits the stress-load on the machine running the cli.")]
    pub concurrency_limit: usize,

    /// The total number of bots of which the process will be dispatched for.
    #[arg(short = 't', long = "total_run_on", default_value = "30", value_hint = clap::ValueHint::Other, long_help = "The total number of bots of which the process will be dispatched for.")]
    pub total_run_on: usize,

    /// Optional path to a SQL file to pull the bots from.
    /// If not provided, the default value is "bots.sql".
    /// And is looked for in the current working directory of the binary.
    #[arg(short = 'f', long = "file", help = "The path to the SQL file.", required = false, default_value = "bots.sql", value_hint = clap::ValueHint::FilePath
    )]
    sql_file: Option<PathBuf>,

    /// Optional verbosity level of the logger.
    /// You may provide this as either a string or a number.
    ///
    /// The least verbose as 0 (Error -> Error Only)
    /// Most verbose as 4 (Trace -> Trace Everything
    /// If not provided, the default value is "INFO".
    #[arg(value_enum, name = "verbosity", short = 'l', long = "level", help = "The verbosity level of the logger.", required = false, default_value = "INFO", value_hint = clap::ValueHint::Other)]
    pub verbosity_level: Option<VerbosityLevel>,

    /// Optional span level of the logger.
    /// You may provide this as either a string or a number.
    /// If not provided, the default value is "NONE".
    ///
    /// From least to most verbose:
    /// -> "NONE" (0) - Do not log any spans.
    /// -> "EXIT" (1) - Only log when exiting a span.
    /// -> "ENTER" (2) - Only log when entering a span.
    /// -> "FULL" (3) - Log both entering and exiting a span.
    #[arg(value_enum, name = "span", short = 's', long = "span", help = "The span level of the logger.", required = false, default_value = "NONE", value_hint = clap::ValueHint::Other)]
    pub span_type: Option<SpanType>,
}

/// The verbosity level of the logger.
///
/// The least verbose as 0 (Error -> Error Only)
/// Most verbose as 4 (Trace -> Trace Everything).
#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
#[clap(name = "VerbosityLevel", rename_all = "upper")]
pub enum VerbosityLevel {
    #[value(name = "ERROR", alias = "error", alias = "Error", alias = "0")]
    Error,
    #[value(name = "WARN", alias = "warn", alias = "Warn", alias = "1")]
    Warn,
    #[value(name = "INFO", alias = "info", alias = "Info", alias = "2")]
    Info,
    #[value(name = "DEBUG", alias = "debug", alias = "Debug", alias = "3")]
    Debug,
    #[value(name = "TRACE", alias = "trace", alias = "Trace", alias = "4")]
    Trace,
}

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
#[clap(name = "SpanType", rename_all = "upper")]
pub enum SpanType {
    #[value(name = "NONE", alias = "none", alias = "None", alias = "0")]
    None,
    #[value(name = "EXIT", alias = "exit", alias = "Exit", alias = "1")]
    Exit,
    #[value(name = "ENTER", alias = "enter", alias = "Enter", alias = "2")]
    Enter,
    #[value(name = "FULL", alias = "full", alias = "Full", alias = "3")]
    Full,
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Cli {
    /// Create a new instance of the Cli struct.
    ///
    /// # Notes:
    /// This will check if the AutomateC executable exists at the path specified in the prelude.
    /// If it does not exist, it will return an error and exit the process.
    ///
    /// There is a bypass for this check, which can be set by setting the environment variable BYPASS_AUTOMATEC_CHECK.
    ///
    /// This is useful for testing purposes.
    pub fn new() -> Self {
        // match check_automate_exists() {
        //     Ok(_) => (),
        //     Err(e) => {
        //         error!("Tracing Error: {}", e);
        //         eprintln!("Error: {}", e);
        //         std::process::exit(1);
        //     }
        // }
        Self::parse()

        // let mut s = Self::parse();
        // if s.sql_file.is_none() {
        //     s.sql_file = Some(PathBuf::from("bots.sql"));
        // }
        // s
    }
    pub fn process(&self) -> &str {
        &self.process
    }

    pub fn concurrency_limit(&self) -> usize {
        self.concurrency_limit
    }

    pub fn total_run_on(&self) -> usize {
        self.total_run_on
    }

    pub fn sql_file(&self) -> &PathBuf {
        self.sql_file.as_ref().unwrap()
    }

    pub fn verbosity_level(&self) -> VerbosityLevel {
        self.verbosity_level.unwrap_or(VerbosityLevel::Info)
    }
}

impl Cli {
    pub fn serialize_sql_file(&self) -> Result<String> {
        info!("SERIALIZE:: Starting serialization...");
        let buffer = std::fs::read(self.sql_file())?;
        let sql_file_query = String::from_utf8(buffer)?.replace("\n", " ");
        Ok(sql_file_query)
    }

    pub fn check_automate_exists(self) -> Result<Self> {
        if std::env::var("BYPASS_AUTOMATEC_CHECK").is_ok() {
            return Ok(self);
        }
        let path = std::path::Path::new(&*DEFAULT_EXE_PATH);
        if !path.exists() {
            return Err(Error::AutomateCNotFound);
        }
        Ok(self)
    }
}

impl From<VerbosityLevel> for tracing_subscriber::filter::EnvFilter {
    fn from(level: VerbosityLevel) -> Self {
        match level {
            VerbosityLevel::Error => tracing_subscriber::filter::EnvFilter::new("ERROR"),
            VerbosityLevel::Warn => tracing_subscriber::filter::EnvFilter::new("WARN"),
            VerbosityLevel::Info => tracing_subscriber::filter::EnvFilter::new("INFO"),
            VerbosityLevel::Debug => tracing_subscriber::filter::EnvFilter::new("DEBUG"),
            VerbosityLevel::Trace => tracing_subscriber::filter::EnvFilter::new("TRACE"),
        }
    }
}

impl From<u8> for VerbosityLevel {
    fn from(level: u8) -> Self {
        match level {
            0 => VerbosityLevel::Error,
            1 => VerbosityLevel::Warn,
            2 => VerbosityLevel::Info,
            3 => VerbosityLevel::Debug,
            4 => VerbosityLevel::Trace,
            _ => VerbosityLevel::Info,
        }
    }
}

impl FromStr for VerbosityLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "ERROR" => Ok(VerbosityLevel::Error),
            "WARN" => Ok(VerbosityLevel::Warn),
            "INFO" => Ok(VerbosityLevel::Info),
            "DEBUG" => Ok(VerbosityLevel::Debug),
            "TRACE" => Ok(VerbosityLevel::Trace),
            _ => Err(Error::Generic(format!(
                "Verbosity level: {} is not supported.",
                s
            ))),
        }
    }
}

impl From<SpanType> for tracing_subscriber::filter::EnvFilter {
    fn from(level: SpanType) -> Self {
        match level {
            SpanType::None => tracing_subscriber::filter::EnvFilter::new("NONE"),
            SpanType::Exit => tracing_subscriber::filter::EnvFilter::new("EXIT"),
            SpanType::Enter => tracing_subscriber::filter::EnvFilter::new("ENTER"),
            SpanType::Full => tracing_subscriber::filter::EnvFilter::new("FULL"),
        }
    }
}

impl From<u8> for SpanType {
    fn from(level: u8) -> Self {
        match level {
            0 => SpanType::None,
            1 => SpanType::Exit,
            2 => SpanType::Enter,
            3 => SpanType::Full,
            _ => SpanType::None,
        }
    }
}

impl FromStr for SpanType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "NONE" => Ok(SpanType::None),
            "ENTER" => Ok(SpanType::Enter),
            "EXIT" => Ok(SpanType::Exit),
            "FULL" => Ok(SpanType::Full),
            _ => Err(Error::Generic(format!(
                "Span type: {} is not supported.",
                s
            ))),
        }
    }
}

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))), // When a command is inc. This is the tag collor for 'Usage:'
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue))), // Main headers in the help menu (e.g. Arguments, Options)
        )
        .literal(
            anstyle::Style::new()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightWhite))), // Strings for args etc { -t, --total }
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red)))
                .effects(anstyle::Effects::ITALIC),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}
