use std::time::Duration;

use clap::{arg, Command};
use simple_logger;
use tokio::time::sleep;

pub mod conf;
pub mod core;

fn main() {
    simple_logger::init().unwrap();

    let run_cmd = Command::new("run").about("to run xray-rust").args([
        arg!(-d --dump "Dump merged config only, without launching Xray server."),
        arg!(-t --test "Test config file only, without launching Xray server."),
        arg!(-f --format "Format of input file."),
        arg!(-c --config "use the config to run"),
    ]);

    let version_cmd = Command::new("version")
        .about("to show the xray-rsut version")
        .aliases(["ver", "v", "V"]);

    let cmd = Command::new("xray-rust")
        .bin_name("xray-rust")
        .author("sofiworker")
        .version("1.0.0")
        .about("this xray-core use rust wirite")
        .subcommand(run_cmd)
        .subcommand(version_cmd);

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("run", _matches)) => {
            let c = conf::AppConfig::get().clone();
            let _ = core::server::Server::new(c).start();
            let _ = sleep(Duration::from_secs(10000));
        }
        Some(("version", _matches)) => println!("v1.0.0"),
        _ => unreachable!("the xray-rust run failed!"),
    };
}
