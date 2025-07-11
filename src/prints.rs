use crate::values::VizValue;
use colored::Colorize;

#[derive(PartialEq, Eq)]
pub enum DisplayOption {
    Key,
    ArrayElement,
    None,
}

pub fn print_object_data(
    name: &str,
    value: VizValue,
    indent: usize,
    indent_step: usize,
    display_option: DisplayOption,
) {
    let indent_str = " ".repeat(indent);

    match value {
        VizValue::Null => match display_option {
            DisplayOption::Key => println!("{}{}: {}", indent_str, name.blue(), "~".bright_black()),
            DisplayOption::ArrayElement => println!("{}- {}", indent_str, "~".bright_black()),
            DisplayOption::None => println!("{}{}", indent_str, "~".bright_black()),
        },
        VizValue::Bool(b) => match display_option {
            DisplayOption::Key => println!("{}{}: {}", indent_str, name.blue(), b),
            DisplayOption::ArrayElement => println!("{}- {}", indent_str, b),
            DisplayOption::None => println!("{}{}", indent_str, b),
        },
        VizValue::Number(n) => match display_option {
            DisplayOption::Key => {
                println!("{}{}: {}", indent_str, name.blue(), n.to_string().red())
            }
            DisplayOption::ArrayElement => println!("{}- {}", indent_str, n.to_string().red()),
            DisplayOption::None => println!("{}{}", indent_str, n.to_string().red()),
        },
        VizValue::Float(f) => match display_option {
            DisplayOption::Key => {
                println!("{}{}: {}", indent_str, name.blue(), f.to_string().red())
            }
            DisplayOption::ArrayElement => println!("{}- {}", indent_str, f.to_string().red()),
            DisplayOption::None => println!("{}{}", indent_str, f.to_string().red()),
        },
        VizValue::String(s) => match display_option {
            DisplayOption::Key => println!("{}{}: \"{}\"", indent_str, name.blue(), s.green()),
            DisplayOption::ArrayElement => println!("{}- \"{}\"", indent_str, s.green()),
            DisplayOption::None => println!("{}\"{}\"", indent_str, s.green()),
        },
        VizValue::Object(map) => {
            // Print the key if this is a key-value pair
            if display_option == DisplayOption::Key {
                println!("{}{}:", indent_str, name.blue());
            } else if display_option == DisplayOption::ArrayElement {
                println!("{}:", indent_str);
            }

            let next_indent = indent + indent_step;
            for (k, v) in map.into_iter() {
                print_object_data(&k, v, next_indent, indent_step, DisplayOption::Key);
            }
        }
        VizValue::Array(vec) => {
            // Print the key if this is a key-value pair
            if display_option == DisplayOption::Key {
                println!("{}{}:", indent_str, name.blue());
            } else if display_option == DisplayOption::ArrayElement {
                println!("{}:", indent_str);
            }

            let next_indent = indent + indent_step;
            for item in vec.into_iter() {
                match item {
                    VizValue::Object(_) | VizValue::Array(_) => {
                        // Complex items: print dash and then the nested structure
                        print!("{}- ", " ".repeat(next_indent));
                        print_object_data(
                            "",
                            item,
                            next_indent + 2,
                            indent_step,
                            DisplayOption::None,
                        );
                    }
                    scalar => {
                        // Scalar items: print with dash prefix
                        print_object_data(
                            "",
                            scalar,
                            next_indent,
                            indent_step,
                            DisplayOption::ArrayElement,
                        );
                    }
                }
            }
        }
    }
}
