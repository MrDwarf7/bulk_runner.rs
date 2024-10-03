use clap::{command, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "BulkRunner.rs",
    about = "A CLI tool to run Blue Prism processes (Via AutomateC dispatch) in bulk.",
    long_about = "\n
    BulkRunner.rs is a CLI tool designed to facilitate the execution of Blue Prism AutomateC processes in bulk. 
    It streamlines the process of launching multiple instances/process via the Control Room, each targeting a distinct resource or bot. 
    This is particularly useful during change over periods where multiple bots need to be transitioned from one process to another.",
    version,
    arg_required_else_help = true
)]
pub struct Cli {
    /// The process to run on all the bots pulled by the SQL query.
    #[arg(
        // 
        index = 1,
        help = "The process to run the bots on.",
        value_hint = clap::ValueHint::Other
    )]
    pub process: String,

    /// The number of bots to run concurrently.
    /// This is handled internally via a semaphore, not via the hardware concurrency of the CPU.
    #[arg(
        short = 't',
        long = "total",
        default_value = "30",
        //index = 2,
        long_help = "The number of bots to run concurrently. This is handled internally via a semaphore, not via the hardware concurrency of the CPU.",
        value_hint = clap::ValueHint::Other
    )]
    pub total_bots: usize,

    /// Optional path to a SQL file to pull the bots from.
    /// If not provided, the default value is "bots.sql".
    /// And is looked for in the current working directory of the binary.
    #[arg(
        short = 'f',
        long = "file",
        help = "The path to the SQL file.",
        default_value = "bots.sql",
        value_hint = clap::ValueHint::FilePath
    )]
    sql_file: Option<PathBuf>,
}

impl Cli {
    pub fn process(&self) -> &str {
        &self.process
    }

    pub fn total_bots(&self) -> usize {
        self.total_bots
    }

    pub fn sql_file(&self) -> &PathBuf {
        self.sql_file.as_ref().unwrap()
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Cli {
    pub fn new() -> Self {
        let mut s = Self::parse();
        if s.sql_file.is_none() {
            s.sql_file = Some(PathBuf::from("bots.sql"));
        }
        s
    }
}
