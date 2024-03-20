pub mod conf;

use clap::{arg, Command};

// use conf;

fn main() {
    let run_cmd = Command::new("run").about("to run xray-rust").args([
        arg!(-d --dump "Dump merged config only, without launching Xray server."),
        arg!(-t --test "Test config file only, without launching Xray server."),
        arg!(-f --format "Format of input file."),
    ]);

    let version_cmd = Command::new("version")
        .about("to show the xray-rsut version")
        .aliases(["ver", "v", "V"]);

    let cmd = Command::new("xray-rust")
        .bin_name("xray")
        .author("sofiworker")
        .version("1.0.0")
        .about("this xray-core use rust wirite")
        .args([arg!(-h --help "to show xray-rust version")])
        .subcommand(run_cmd)
        .subcommand(version_cmd);

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("run", _matches)) => {
            println!("{}", 1);
            conf::show();
        }
        Some(("version", _matches)) => println!("v1.0.0"),
        _ => unreachable!("the xray-rust run failed!"),
    };
}
