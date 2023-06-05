use clap::{Parser, Subcommand, ValueEnum};
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Action {
    /// Log monitored changes to standard output pipe
    LogOnly,
    /// Do nothing
    None,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Action::LogOnly => {
                    "log-only"
                }
                Action::None => {
                    "none"
                }
            }
        )
    }
}

//#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
/// Monitor type
#[derive(Subcommand, Debug, Copy, Clone)]
pub enum MonitorType {
    /// Summary for each interval
    Debounce {
        /// Duration in seconds monitor will run
        interval: u64,
    },
    /// Notify as soon as change was made
    Async,
}

impl Display for MonitorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MonitorType::Debounce { interval } => {
                write!(f, "debounced [each {}s]", interval)
            }
            MonitorType::Async => {
                write!(f, "async")
            }
        }
    }
}

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Params {
    /// The action applied to an detected change
    #[arg(short, long, value_name = "ACTION")]
    #[arg(default_value_t = Action::LogOnly)]
    pub action: Action, //Option<Action>,
    /// Log monitored changes to file
    #[arg(short, long, value_name = "FILE")]
    pub log_file: Option<String>,
    /// The path to the directory to monitor
    #[arg(short, long, value_name = "FILE")]
    #[arg(default_value_t = String::from("."))]
    pub path: String,
    #[command(subcommand)]
    pub monitor_type: MonitorType,
}

pub struct Conf {
    param: Params,
}

impl Conf {
    pub fn new() -> Self {
        Self {
            param: Params::parse(),
        }
    }

    pub fn get_params(&self) -> Params {
        self.param.clone()
    }
}
