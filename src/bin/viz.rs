use colored::Colorize;
use std::{fs, path::Path, process::exit};
use viz::{
    args::build_cli,
    processors::{json::JSONProcessor, toml::TOMLProcessor, yaml::YAMLProcessor, Processor},
    terminal::Messages,
    values::VizValue,
};

fn print_object_data(name: &str, object: VizValue, indent: usize, initial_indent: usize) {
    let indent_str = " ".repeat(indent);
    match object {
        VizValue::Null => println!(
            "{}{}: {}",
            indent_str,
            name.bold().white(),
            "null".bright_black()
        ),
        VizValue::Bool(b) => println!(
            "{}{}: {}",
            indent_str,
            name.bold().white(),
            b.to_string().blue()
        ),
        VizValue::Number(n) => println!(
            "{}{}: {}",
            indent_str,
            name.bold().white(),
            n.to_string().red()
        ),
        VizValue::Float(f) => println!(
            "{}{}: {}",
            indent_str,
            name.bold().white(),
            f.to_string().red()
        ),
        VizValue::String(s) => println!("{}{}: {}", indent_str, name.bold().white(), s.yellow()),
        VizValue::Object(map) => {
            println!("{}{}: ", indent_str, name.bold().white());
            for (key, val) in map {
                print_object_data(&key, val, indent + initial_indent, initial_indent);
            }
        }
        VizValue::Array(vec) => {
            println!("{}{}: ", indent_str, name.bold().white());
            for (i, item) in vec.into_iter().enumerate() {
                print_object_data(&format!("[{}]", i), item, indent + initial_indent, initial_indent);
            }
        }
    }
}

fn main() {
    let args = build_cli().get_matches();
    let file = args.get_one::<String>("path").unwrap();

    let file_path = Path::new(file);
    let indent = args
        .get_one::<usize>("indent")
        .unwrap_or(&2)
        .to_owned();

    if indent > 10 {
        Messages::error("Indentation level must be less than or equal to 10.");
        exit(1);
    }

    if indent == 0 {
        Messages::error("Indentation level must be greater than 0.");
        exit(1);
    }

    if !file_path.exists() {
        Messages::error("file not found.");
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
            Messages::error("unsupported file format.");
            exit(1);
        }
    };

    if let Err(e) = parsed_data {
        Messages::error(&format!("{}", e));
        exit(1);
    } 

    if let VizValue::Object(map) = parsed_data.unwrap() {
        for (key, val) in map {
            print_object_data(&key, val, 0, indent);
        }
    } else {
        Messages::error("internal error: parsed data is not a valid object.");
        exit(1);
    }
}
