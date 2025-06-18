use std::env;

use clap::{value_parser, Arg, Command};

/// A CLI builder for Viz app.
pub fn build_cli() -> Command {
    Command::new("vel")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .args([
            Arg::new("path")
                .help("Path to the file to view.")
                .value_parser(value_parser!(String))
                .required(false),
            Arg::new("language")
                .short('l')
                .long("language")
                .help("Language of the file to view (e.g., json, toml, yaml).")
                .value_parser(value_parser!(String))
                .required(false),
            Arg::new("no-color")
                .short('n')
                .long("no-color")
                .help("Disable colored output.")
                .action(clap::ArgAction::SetTrue)
                .required(false),
            Arg::new("indent")
                .short('i')
                .long("indent")
                .help("Indentation level for output.")
                .value_parser(value_parser!(usize))
                .default_value("2")
                .required(false),
        ])
}
