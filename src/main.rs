mod filesystem;

use crate::filesystem::monitor::{monitor_path_async, monitor_path_debounced};
use std::path::Path;

mod tools;
use crate::tools::{
    config::{Conf, MonitorType},
    output::print_sep_line,
};

//use crate::tools::output::print_sep_line;
//use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_sep_line();
    //let args = Cli::parse();
    let conf: Conf = Conf::new();
    let params = conf.get_params();
    println!("action  : {}", params.action);
    println!("monitor : {}", params.monitor_type);
    println!("path    : {}", params.path);
    print_sep_line();

    match &params.monitor_type {
        MonitorType::Debounce { interval } => {
            monitor_path_debounced(&Path::new(&params.path), *interval)?;
        }
        MonitorType::Async => {
            monitor_path_async(&Path::new(&params.path))?;
        }
    }

    Ok(())
}
