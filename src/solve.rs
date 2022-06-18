use mastermind::mastermind::*;

use colored::Colorize;

pub fn run() {
    println!("{}", "Solve mode\n".bold());
    println!("The following colors are available:");
    for color in Colors::all() {
        println!("{}: {} ({})", color.prettify(), color.to_string(), color.to_char())
    }
}
