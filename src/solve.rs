use mastermind::mastermind::*;

use colored::Colorize;

pub fn run() {
    println!("{}", "Solve mode\n".bold());

    println!("The following colors are available:");
    for color in Colors::all() {
        println!(
            "{}: {} ({})",
            color.prettify(),
            color.to_string(),
            color.to_char()
        );
    }

    println!("The following match levels are available:");
    for match_level in MatchLevels::all() {
        println!(
            "{}: {} ({})",
            match_level.prettify(),
            match_level.to_string(),
            match_level.to_char()
        );
    }
}
