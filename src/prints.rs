use crate::values::VizValue;
use colored::Colorize;

/// The enum that indicates how to display key.
#[derive(Eq, PartialEq)]
pub enum DisplayType {
    /// Displays with key name and it's value
    Key,

    /// Only displays it's value
    ArrayElement,
}

/// Prints VizValue in Prettij markup language.
pub fn print_prettij(
    name: &str,
    value: VizValue,
    initial_indent: usize,
    indent_step: usize,
    display_type: DisplayType,
    is_last: bool,
) {
    let indent_str = " ".repeat(initial_indent);
    match value {
        VizValue::String(s) => match display_type {
            DisplayType::Key => {
                println!("{}{} = \"{}\"", indent_str, name.blue(), s.green(),);
            }
            DisplayType::ArrayElement => {
                println!(
                    "{}\"{}\"{}",
                    indent_str,
                    s.green(),
                    if !is_last { "," } else { "" }
                );
            }
        },
        VizValue::Number(n) => match display_type {
            DisplayType::Key => {
                println!("{}{} = {}", indent_str, name.blue(), n.to_string().red());
            }
            DisplayType::ArrayElement => {
                println!(
                    "{}{}{}",
                    indent_str,
                    n.to_string().red(),
                    if !is_last { "," } else { "" }
                );
            }
        },
        VizValue::Float(f) => match display_type {
            DisplayType::Key => {
                println!("{}{} = {}", indent_str, name.blue(), f.to_string().red());
            }
            DisplayType::ArrayElement => {
                println!(
                    "{}{}{}",
                    indent_str,
                    f.to_string().red(),
                    if !is_last { "," } else { "" }
                );
            }
        },
        VizValue::Null => match display_type {
            DisplayType::Key => {
                println!("{}{} = {}", indent_str, name.blue(), "null".bright_black());
            }
            DisplayType::ArrayElement => {
                println!(
                    "{}{}{}",
                    indent_str,
                    "null".bright_black(),
                    if !is_last { "," } else { "" }
                );
            }
        },
        VizValue::Bool(b) => match display_type {
            DisplayType::Key => {
                println!(
                    "{}{} = {}",
                    indent_str,
                    name.blue(),
                    b.to_string().bright_magenta()
                );
            }
            DisplayType::ArrayElement => {
                println!(
                    "{}{}{}",
                    indent_str,
                    b.to_string().bright_magenta(),
                    if !is_last { "," } else { "" }
                );
            }
        },
        VizValue::Array(vec) => {
            if display_type == DisplayType::Key {
                println!("{indent_str}{} = [", name.blue());
            } else {
                println!("{indent_str}[");
            }

            let next_indent = initial_indent + indent_step;
            let vec_len = vec.len();
            for (id, item) in vec.into_iter().enumerate() {
                print_prettij(
                    "",
                    item,
                    next_indent,
                    indent_step,
                    DisplayType::ArrayElement,
                    id == vec_len - 1,
                );
            }
            println!("{indent_str}]");
        }
        VizValue::Object(map) => {
            if display_type == DisplayType::Key {
                println!("{indent_str}{} = {{", name.blue());
            } else {
                println!("{indent_str}{{");
            }

            let next_indent = initial_indent + indent_step;
            let map_len = map.len();
            for (id, (k, v)) in map.into_iter().enumerate() {
                print_prettij(
                    &k,
                    v,
                    next_indent,
                    indent_step,
                    DisplayType::Key,
                    id == map_len - 1,
                );
            }
            println!("{indent_str}}}{}", if !is_last { "," } else { "" });
        }
    }
}
