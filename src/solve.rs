use mastermind::mastermind::*;

use colored::Colorize;

pub fn run() {
    println!("{}", "Solve mode".bold());
    println!("The following colors are available:\n");
    for color in Colors::all() {
        println!("{}: {}", color.prettify(), color.to_string())
    }
}
