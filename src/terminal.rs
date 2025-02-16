use colored::Colorize;

pub struct Messages;

impl Messages {
    pub fn error(message: &str) {
        println!("{}: {}", "error".red().bold(), message);
    }
}
