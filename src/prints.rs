use crate::values::VizValue;
use colored::Colorize;
use indexmap::IndexMap;

pub fn print_object_data(
    name: &str,
    object: VizValue,
    indent: usize,
    initial_indent: usize,
    is_last: bool,
    print_name: bool,
) {
    let indent_str = " ".repeat(indent);
    let name_prefix = format_name_prefix(name, print_name);
    let comma_suffix = format_comma_suffix(is_last);

    match object {
        VizValue::Null => {
            println!(
                "{}{}{}{}",
                indent_str,
                name_prefix,
                "null".bright_black(),
                comma_suffix
            );
        }
        VizValue::Bool(b) => {
            println!("{}{}{}{}", indent_str, name_prefix, b, comma_suffix);
        }
        VizValue::Number(n) => {
            println!(
                "{}{}{}{}",
                indent_str,
                name_prefix,
                n.to_string().red(),
                comma_suffix
            );
        }
        VizValue::Float(f) => {
            println!(
                "{}{}{}{}",
                indent_str,
                name_prefix,
                f.to_string().red(),
                comma_suffix
            );
        }
        VizValue::String(s) => {
            println!(
                "{}{}\"{}\"{}",
                indent_str,
                name_prefix,
                s.green(),
                comma_suffix
            );
        }
        VizValue::Object(map) => {
            print_object(
                &indent_str,
                &name_prefix,
                map,
                indent,
                initial_indent,
                is_last,
            );
        }
        VizValue::Array(vec) => {
            print_array(&indent_str, name, vec, indent, initial_indent, is_last);
        }
    }
}

fn format_name_prefix(name: &str, print_name: bool) -> String {
    if print_name {
        format!("\"{}\": ", name.blue())
    } else {
        String::new()
    }
}

fn format_comma_suffix(is_last: bool) -> &'static str {
    if is_last {
        ""
    } else {
        ","
    }
}

fn print_object(
    indent_str: &str,
    name_prefix: &str,
    map: IndexMap<String, VizValue>,
    indent: usize,
    initial_indent: usize,
    is_last: bool,
) {
    println!("{}{}{{", indent_str, name_prefix);

    let entries: Vec<_> = map.into_iter().collect();
    let total = entries.len();

    for (i, (key, val)) in entries.into_iter().enumerate() {
        let is_last_entry = i + 1 == total;
        print_object_data(
            &key,
            val,
            indent + initial_indent,
            initial_indent,
            is_last_entry,
            true,
        );
    }

    println!("{}}}{}", indent_str, format_comma_suffix(is_last));
}

fn print_array(
    indent_str: &str,
    name: &str,
    vec: Vec<VizValue>,
    indent: usize,
    initial_indent: usize,
    is_last: bool,
) {
    println!("{}\"{}\": [", indent_str, name.blue());

    let total = vec.len();
    for (i, item) in vec.into_iter().enumerate() {
        let is_last_entry = i + 1 == total;
        print_object_data(
            &format!("[{}]", i + 1),
            item,
            indent + initial_indent,
            initial_indent,
            is_last_entry,
            false,
        );
    }

    println!("{}]{}", indent_str, format_comma_suffix(is_last));
}
