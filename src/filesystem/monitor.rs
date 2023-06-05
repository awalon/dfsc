//use std::{fs, thread, time::Duration};
use std::error::Error;
//use std::fmt::{Debug, Error};
use std::path::Path;
use std::time::Duration;

use notify::{RecursiveMode, Watcher}; // debounce
use notify_debouncer_full::new_debouncer;
//use tempfile::tempdir;

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher}; // + async

use crate::tools::output::{print_event, print_events, print_sep_line};

pub fn monitor_path_debounced(path: &Path, seconds: u64) -> Result<(), Box<dyn Error>> {
    println!(
        "monitoring path debounced [each {}s]: {}",
        seconds,
        path.display()
    );
    print_sep_line();

    println!("|object|kind|");

    //fn main() -> Result<(), Box<dyn std::error::Error>> {
    //   let dir = tempdir()?;
    //   let dir_path = dir.path().to_path_buf();
    /*
        // emit some events by changing a file
        thread::spawn(move || {
            let mut n = 1;
            let mut file_path = dir_path.join(format!("file-{n:03}.txt"));
            loop {
                for _ in 0..5 {
                    fs::write(&file_path, b"Lorem ipsum").unwrap();
                    thread::sleep(Duration::from_millis(500));
                }
                n += 1;
                let target_path = dir_path.join(format!("file-{n:03}.txt"));
                fs::rename(&file_path, &target_path).unwrap();
                file_path = target_path;
            }
        });
    */
    // setup debouncer
    let (tx, rx) = std::sync::mpsc::channel();

    // no specific tickrate, max debounce time 2 seconds
    let mut debouncer = new_debouncer(Duration::from_secs(seconds), None, tx)?;

    debouncer.watcher().watch(path, RecursiveMode::Recursive)?;

    debouncer.cache().add_root(path, RecursiveMode::Recursive);

    // print all events and errors
    for result in rx {
        match result {
            Ok(events) => print_events(events),
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        }
        println!();
    }

    Ok(())
}

/// Async, futures channel based event watching
pub fn monitor_path_async(path: &Path) -> notify::Result<()> {
    // -> Result<(), Box<dyn Error>> {
    println!("monitoring path async: {:?}", path);
    print_sep_line();

    println!("|object|kind|");

    futures::executor::block_on(async {
        if let Err(e) = async_watch(path).await {
            println!("error: {:?}", e);
        }
    });

    Ok(())
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => print_event(&event),
            Err(e) => println!("+++ watch error: {:?}", e),
        }
    }

    Ok(())
}
