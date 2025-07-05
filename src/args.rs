use std::env;

use clap::Parser;

/// A CLI builder for Vizo app.
#[derive(Parser, Debug)]
#[command(name = "vizo", about = env!("CARGO_PKG_DESCRIPTION"), version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Path to the file to view
    pub path: Option<String>,

    /// Language of the file to view (e.g., json, toml, yaml).
    #[arg(short, long)]
    pub language: Option<String>,

    /// Disable colored output.
    #[arg(short = 'n', long = "no-color", action = clap::ArgAction::SetTrue)]
    pub no_color: bool,

    /// Indentation level for output.
    #[arg(short, long, default_value_t = 2)]
    pub indent: usize,
}
