use crate::values::VizValue;
use colored::Colorize;

pub fn print_object_data(
    name: &str,
    object: VizValue,
    indent: usize,
    initial_indent: usize,
    is_last: bool,
    print_name: bool,
) {
    let indent_str = " ".repeat(indent);
    match object {
        VizValue::Null => println!(
            "{}{}{}{}",
            indent_str,
            if print_name {
                format!("\"{}\": ", name.blue())
            } else {
                "".into()
            },
            "null".bright_black(),
            if is_last { "" } else { "," }
        ),
        VizValue::Bool(b) => println!(
            "{}{}{}{}",
            indent_str,
            if print_name {
                format!("\"{}\": ", name.blue())
            } else {
                "".into()
            },
            b,
            if is_last { "" } else { "," }
        ),
        VizValue::Number(n) => println!(
            "{}{}{}{}",
            indent_str,
            if print_name {
                format!("\"{}\": ", name.blue())
            } else {
                "".into()
            },
            n.to_string().red(),
            if is_last { "" } else { "," }
        ),
        VizValue::Float(f) => println!(
            "{}{}{}{}",
            indent_str,
            if print_name {
                format!("\"{}\": ", name.blue())
            } else {
                "".into()
            },
            f.to_string().red(),
            if is_last { "" } else { "," }
        ),
        VizValue::String(s) => println!(
            "{}{}\"{}\"{}",
            indent_str,
            if print_name {
                format!("\"{}\": ", name.blue())
            } else {
                "".into()
            },
            s.green(),
            if is_last { "" } else { "," }
        ),
        VizValue::Object(map) => {
            println!(
                "{}{}{{",
                indent_str,
                if print_name {
                    format!("\"{}\": ", name.blue())
                } else {
                    "".into()
                },
            );
            let entries: Vec<_> = map.into_iter().collect();
            let total = entries.len();
            for (i, (key, val)) in entries.into_iter().enumerate() {
                let last = i + 1 == total;
                print_object_data(
                    &key,
                    val,
                    indent + initial_indent,
                    initial_indent,
                    last,
                    true,
                );
            }
            println!("{}}}{}", indent_str, if is_last { "" } else { "," });
        }
        VizValue::Array(vec) => {
            println!("{}\"{}\": [", indent_str, name.blue());
            let total = vec.len();
            for (i, item) in vec.into_iter().enumerate() {
                let last = i + 1 == total;
                print_object_data(
                    &format!("[{}]", i + 1),
                    item,
                    indent + initial_indent,
                    initial_indent,
                    last,
                    false,
                );
            }
            println!("{}]{}", indent_str, if is_last { "" } else { "," });
        }
    }
}
