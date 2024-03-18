use clap::{Command, arg};

fn main() {

    let run_cmd = Command::new("run")
    .about("to run xray-rust")
    .args([
        arg!(-d --dump "Dump merged config only, without launching Xray server."),
        arg!(-t --test "Test config file only, without launching Xray server."),
        arg!(-f --format "Format of input file."),
    ]);

    let cmd = Command::new("xray-rust")
        .bin_name("xray")
        .author("sofiworker")
        .version("1.0.0")
        .about("this xray-core use rust wirite")
        .subcommand(run_cmd);

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("run", matches)) => println!("{}", 1),
        _ => unreachable!("the xray-rust run failed!"),
    };
}
 