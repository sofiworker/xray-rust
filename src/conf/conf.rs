#![allow(deprecated)]
use config::{Config, File};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::RwLock;
use std::time::Duration;

lazy_static::lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(File::with_name("./config.json")).unwrap();

        settings
    });
}

pub fn show() {
    println!(
        " * Settings :: \n\x1b[31m{:?}\x1b[0m",
        SETTINGS
            .read()
            .unwrap()
            .clone()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );
}

fn watch() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        notify::Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(
            Path::new("./config.json"),
            RecursiveMode::NonRecursive,
        )
        .unwrap();

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(Ok(Event {
                kind: notify::event::EventKind::Modify(_),
                ..
            })) => {
                println!(" * Settings.toml written; refreshing configuration ...");
                SETTINGS.write().unwrap().refresh().unwrap();
                show();
            }

            Err(e) => println!("watch error: {:?}", e),

            _ => {
                // Ignore event
            }
        }
    }
}