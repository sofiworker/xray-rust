use clap::{arg, Command};


pub fn get_run_cmd() -> Command {
    Command::new("run").about("to run xray-rust").aliases(["start"]).args([
        arg!(-d --dump "Dump merged config only, without launching Xray server."),
        arg!(-t --test "Test config file only, without launching Xray server."),
        arg!(-f --format "Format of input file."),
        arg!(-c --config "use the config to run"),
    ])
}

pub struct Run {

}

impl Run {

}