use colored::Colorize;
use vizo::app::run;

fn main() {
    if let Err(e) = run() {
        println!("{}: {}", "error".red().bold(), e);
    }
}
