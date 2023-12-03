fn main() {
    let cmd = clap::Command::new("xray-rust")
        .bin_name("xray")
        .subcommand_required(true)
        .subcommand(
            clap::command!("config").arg(
                clap::arg!(--"manifest-path" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf)),
            ),
        );
    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("example", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };
    let config_path = matches.get_one::<std::path::PathBuf>("config-path");
    println!("{config_path:?}");
}
