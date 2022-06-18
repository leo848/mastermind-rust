use mastermind::mastermind::*;
use mastermind::{self, Code};

use clap::ArgMatches;
use colored::Colorize;
use std::{
    io::{stdin, stdout, Write},
    thread,
    time::Duration,
};

pub fn run(matches: &ArgMatches) {
    println!("{}", "Play mode\n".bold());

    if !matches.is_present("no-info") {
        share::print_colors();
    }

    let actual_code = Code::random();
    let mut guess = Code::empty();
    let mut guesses: Vec<solver::Guess> = Vec::new();
    let mut counter = 0;

    while guess != actual_code {
        counter += 1;

        println!();
        print!("Enter guess #{}: ", counter);
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        guess = Code::from_guess_string(&input.trim());

        print!("\x1b[1A\x1b[0J{}\t\t", guess.prettify());
        stdout().flush().unwrap();

        let code_match = actual_code.match_code(&guess);
        guesses.push(solver::Guess((guess, code_match)));

        thread::sleep(Duration::from_millis(300));
        for code_match in code_match.iter() {
            print!("{} ", code_match.prettify());
            stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(300));
        }
        
        if matches.is_present("show-possible") {
            print!(" \x1b[2;4m{}\x1b[0m", solver::possible_codes(&guesses).len()); 
        }
    }

    println!(
        "\n\n{} {}",
        "Solved!".green().bold(),
        format!("You needed {} tries", counter)
    );
}
