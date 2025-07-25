use crate::args::Cli;
use crate::prints::DisplayType;

#[allow(unused_imports)]
use crate::processors::*;

use crate::values::VizValue;
use anyhow::{Result, anyhow, bail};
use clap::Parser;
use colored::Colorize;
use std::env::var;
use std::fs;
use std::io::{Read, stdin};
use std::path::Path;
use std::process::exit;

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    configure_colors(&cli);

    if cli.formats {
        print_formats();
    }

    let (contents, extension) = get_content_and_extension(&cli)?;
    let indent = get_indent(&cli)?;
    let data = get_parsed_data(&contents, &extension)?;

    print_parsed_data(data, indent);

    Ok(())
}

fn print_formats() {
    let features = get_enabled_formats();

    if features.is_empty() {
        println!("Your build of Vizo does not contain any format parsers.");
        exit(1);
    }

    println!("Supported formats:");

    for f in features {
        println!("  {f}");
    }

    exit(0);
}

fn get_enabled_formats() -> Vec<String> {
    let mut features = Vec::with_capacity(3);

    if cfg!(feature = "json") {
        features.push("json".to_string());
    }

    if cfg!(feature = "yaml") {
        features.push("yaml".to_string());
    }

    if cfg!(feature = "toml") {
        features.push("toml".to_string());
    }

    features
}

fn configure_colors(cli: &Cli) {
    let no_color = var("NO_COLOR");

    if no_color.is_ok() {
        colored::control::set_override(false);
    } else {
        colored::control::set_override(!cli.no_color);
    }
}

fn get_content_and_extension(cli: &Cli) -> Result<(String, String)> {
    let file_path = cli.path.clone().unwrap_or_default();

    if file_path.is_empty() {
        get_from_stdin(cli)
    } else {
        get_file_content(&file_path)
    }
}

fn get_from_stdin(cli: &Cli) -> Result<(String, String)> {
    let mut contents = String::new();
    stdin()
        .read_to_string(&mut contents)
        .map_err(|e| anyhow!("failed to read from stdin: {}", e.to_string()))?;

    if let Some(lang) = &cli.language {
        Ok((contents, lang.clone()))
    } else {
        bail!("language is not specified for stdin")
    }
}

fn get_file_content(file_path: &str) -> Result<(String, String)> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(anyhow!("file not found"));
    }

    let contents = fs::read_to_string(file_path)
        .map_err(|e| anyhow!("failed to read file: {}", e.to_string()))?;

    let ext = path
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .to_lowercase();

    Ok((contents, ext))
}

fn get_indent(cli: &Cli) -> Result<usize> {
    let indent = &cli.indent;

    if *indent > 10 {
        return Err(anyhow!("indentation level must be less than or equal 10."));
    }

    Ok(*indent)
}

#[allow(unused_variables)]
fn get_parsed_data(contents: &str, extension: &str) -> Result<VizValue> {
    let parsed_data = match extension {
        #[cfg(feature = "json")]
        "json" => json::JSONProcessor::process_data(contents),
        #[cfg(feature = "toml")]
        "toml" => toml::TOMLProcessor::process_data(contents),
        #[cfg(feature = "yaml")]
        "yaml" | "yml" => yaml::YAMLProcessor::process_data(contents),
        _ => Err(anyhow!("unsupported file format.")),
    }?;

    Ok(parsed_data)
}

fn print_parsed_data(data: VizValue, indent: usize) {
    if let VizValue::Object(map) = data {
        let entries: Vec<_> = map.into_iter().collect();
        for (key, val) in entries.into_iter() {
            crate::prints::print_prettij(&key, val, 0, indent, DisplayType::Key, true);
        }
    } else {
        println!(
            "{}: parsed data is not a valid object.",
            "error".red().bold()
        );
        exit(1);
    }
}
