use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use clap::{Command};
use simple_logger;

mod conf;
mod core;
mod cmd;
mod proxy;

#[tokio::main]
async fn main() {
    simple_logger::init().unwrap();

    let version_cmd = Command::new("version")
        .about("to show the xray-rsut version")
        .aliases(["ver", "v", "V"]);

    let cmd = Command::new("xray-rust")
        .bin_name("xray-rust")
        .author("sofiworker")
        .about("this xray-core use rust wirite")
        .subcommand(cmd::run::get_run_cmd())
        .subcommand(version_cmd);

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("run", _matches)) => {
            let app = core::app::App::new();
            app.start().await.unwrap();
        }
        Some(("version", _matches)) => {
            let v = cmd::version::Version::new();
            v.show();
        }
        _ => unreachable!("the xray-rust run failed!"),
    };


    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).unwrap();
    while !term.load(Ordering::Relaxed) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await
        }
    }
}
