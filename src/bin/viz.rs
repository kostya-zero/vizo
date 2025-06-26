use colored::Colorize;
use viz::app::run;

fn main() {
    if let Err(e) = run() {
        println!("{}: {}", "error".red().bold(), e);
    }
}
