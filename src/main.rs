mod filesystem;

use crate::filesystem::monitor::Monitor;
use std::path::Path;

mod tools;
use crate::tools::{
    config::{Conf, MonitorType},
    output::Output,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let args = Cli::parse();
    let conf: Conf = Conf::new();
    let params = conf.get_params();
    let mut out: Output = Output::new();

    out.print_sep_line();
    println!("action  : {}", params.action);
    if let Some(file) = params.log_file {
        println!("log file: {}", file);
        out.set_file(file)
    }
    println!("monitor : {}", params.monitor_type);
    println!("path    : {}", params.path);
    out.print_sep_line();

    let mon: Monitor = Monitor::new(out);
    match &params.monitor_type {
        MonitorType::Debounce { interval } => {
            mon.monitor_path_debounced(&Path::new(&params.path), *interval)?;
        }
        MonitorType::Async => {
            mon.monitor_path_async(&Path::new(&params.path))?;
        }
    }

    Ok(())
}
