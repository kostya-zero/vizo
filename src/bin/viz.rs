use std::{fs, path::Path, process::exit};
use viz::{
    args::build_cli,
    processors::{json::JSONProcessor, toml::TOMLProcessor, yaml::YAMLProcessor, Processor},
    terminal::Messages,
    values::VizValue,
};

fn main() {
    let args = build_cli().get_matches();

    // Always unwrap because we require the path argument.
    let file = args.get_one::<String>("path").unwrap();

    let file_path = Path::new(file);
    let indent = args.get_one::<usize>("indent").unwrap_or(&2).to_owned();

    if indent > 10 {
        Messages::error("Indentation level must be less than or equal to 10.");
        exit(1);
    }

    if !file_path.exists() {
        Messages::error("file not found.");
        exit(1);
    }

    let contents = fs::read_to_string(file_path).unwrap();
    let ext = if let Some(ext) = args.get_one::<String>("language") {
        ext.to_lowercase()
    } else {
        file_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_lowercase()
    };

    let parsed_data = match ext.as_str() {
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
            viz::prints::print_object_data(&key, val, indent, indent, last, true);
        }
        println!("}}");
    } else {
        Messages::error("internal error: parsed data is not a valid object.");
        exit(1);
    }
}
