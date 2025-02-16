use clap::{value_parser, Arg, Command};

/// A CLI builder for Viz app.
pub fn build_cli() -> Command {
    Command::new("vel")
        .about("Structured files viewer.")
        .version("0.1.0")
        .arg_required_else_help(true)
        .args([Arg::new("path")
            .help("Path to the file to view.")
            .value_parser(value_parser!(String))
            .num_args(1)
            .required(true)])
}
