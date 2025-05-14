use colored::Colorize;
use std::{fs, path::Path, process::exit};
use viz::{
    args::build_cli,
    processors::{json::JSONProcessor, toml::TOMLProcessor, yaml::YAMLProcessor, Processor},
    terminal::Messages,
    values::VizValue,
};

fn print_object_data(
    name: &str,
    object: VizValue,
    indent: usize,
    initial_indent: usize,
    is_last: bool,
) {
    let indent_str = " ".repeat(indent);
    match object {
        VizValue::Null => println!(
            "{}\x1b[94m\"{}\"\x1b[0m: {}{}",
            indent_str,
            name,
            "null".bright_black(),
            if is_last { "" } else { "," }
        ),
        VizValue::Bool(b) => println!(
            "{}\x1b[94m\"{}\"\x1b[0m: {}{}",
            indent_str,
            name,
            b,
            if is_last { "" } else { "," }
        ),
        VizValue::Number(n) => println!(
            "{}\x1b[94m\"{}\"\x1b[0m: {}{}",
            indent_str,
            name,
            n.to_string().red(),
            if is_last { "" } else { "," }
        ),
        VizValue::Float(f) => println!(
            "{}\x1b[94m\"{}\"\x1b[0m: {}{}",
            indent_str,
            name,
            f.to_string().red(),
            if is_last { "" } else { "," }
        ),
        VizValue::String(s) => println!(
            "{}\x1b[94m\"{}\"\x1b[0m: \x1b[92m\"{}\"\x1b[0m{}",
            indent_str,
            name,
            s,
            if is_last { "" } else { "," }
        ),
        VizValue::Object(map) => {
            println!("{}\x1b[94m\"{}\"\x1b[0m: {{", indent_str, name);
            let entries: Vec<_> = map.into_iter().collect();
            let total = entries.len();
            for (i, (key, val)) in entries.into_iter().enumerate() {
                let last = i + 1 == total;
                print_object_data(&key, val, indent + initial_indent, initial_indent, last);
            }
            println!("{} }}{}", indent_str, if is_last { "" } else { "," });
        }
        VizValue::Array(vec) => {
            println!("{}\x1b[94m\"{}\"\x1b[0m: [", indent_str, name);
            let total = vec.len();
            for (i, item) in vec.into_iter().enumerate() {
                let last = i + 1 == total;
                print_object_data(
                    &format!("[{}]", i),
                    item,
                    indent + initial_indent,
                    initial_indent,
                    last,
                );
            }
            println!("{} ]{}", indent_str, if is_last { "" } else { "," });
        }
    }
}

fn main() {
    let args = build_cli().get_matches();
    let file = args.get_one::<String>("path").unwrap();

    let file_path = Path::new(file);
    let indent = args.get_one::<usize>("indent").unwrap_or(&2).to_owned();

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
        println!("{{");
        let entries: Vec<_> = map.into_iter().collect();
        let total = entries.len();
        for (i, (key, val)) in entries.into_iter().enumerate() {
            let last = i + 1 == total;
            print_object_data(&key, val, indent, indent, last);
        }
        println!("}}");
    } else {
        Messages::error("internal error: parsed data is not a valid object.");
        exit(1);
    }
}
