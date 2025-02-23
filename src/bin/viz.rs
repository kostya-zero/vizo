use colored::Colorize;
use std::{fs, path::Path, process::exit};
use viz::{
    args::build_cli,
    processors::{json::JSONProcessor, toml::TOMLProcessor, yaml::YAMLProcessor, Processor},
    terminal::Messages,
    values::VizValues,
};

fn print_object_data(name: &str, object: VizValues, ident: usize) {
    let indent_str = " ".repeat(ident);
    match object {
        VizValues::Null => println!(
            "{}{}: {}",
            indent_str,
            name.bold().white(),
            "null".bright_black()
        ),
        VizValues::Bool(b) => println!(
            "{}{}: {}",
            indent_str,
            name.bold().white(),
            b.to_string().blue()
        ),
        VizValues::Number(n) => println!(
            "{}{}: {}",
            indent_str,
            name.bold().white(),
            n.to_string().red()
        ),
        VizValues::Float(f) => println!(
            "{}{}: {}",
            indent_str,
            name.bold().white(),
            f.to_string().red()
        ),
        VizValues::String(s) => println!("{}{}: {}", indent_str, name.bold().white(), s.yellow()),
        VizValues::Object(map) => {
            println!("{}{}: ", indent_str, name.bold().white());
            for (key, val) in map {
                print_object_data(&key, val, ident + 2);
            }
        }
        VizValues::Array(vec) => {
            println!("{}{}: ", indent_str, name.bold().white());
            for (i, item) in vec.into_iter().enumerate() {
                print_object_data(&format!("[{}]", i), item, ident + 2);
            }
        }
    }
}

fn main() {
    let args = build_cli().get_matches();
    let file = args.get_one::<String>("path").unwrap();

    let file_path = Path::new(file);

    if !file_path.exists() {
        Messages::error("File not found!");
        exit(1);
    }

    let contents = fs::read_to_string(file_path).unwrap();
    let ext = file_path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("");
    let parsed_data = match ext {
        "json" => JSONProcessor::process_data(&contents),
        "toml" => TOMLProcessor::process_data(&contents),
        "yaml" | "yml" => YAMLProcessor::process_data(&contents),
        _ => {
            Messages::error("Unsupported file format!");
            exit(1);
        }
    };

    if let Err(e) = parsed_data {
        Messages::error(&format!("{}", e));
        exit(1);
    } 

    if let VizValues::Object(map) = parsed_data.unwrap() {
        for (key, val) in map {
            print_object_data(&key, val, 0);
        }
    } else {
        Messages::error("Parsed data is not a valid object.");
        exit(1);
    }
}
