use mastermind::mastermind::*;
use mastermind::{self, Code};

use clap::ArgMatches;
use colored::Colorize;
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

pub fn run(matches: &ArgMatches) {
    println!("{}", "Play mode\n".bold());

    if !matches.is_present("no-info") {
        share::print_colors();
    }

    let actual_code = match matches.value_of("given-code") {
        None => Code::random(),
        Some(code) => Code::from_guess_string(code),
    };

    let mut guess = None::<Code>;
    let mut guesses: Vec<solver::Guess> = Vec::new();
    let mut counter = 0;

    while guess.is_none() || guess.unwrap() != actual_code {
        counter += 1;

        println!();
        print!("Enter guess #{}: ", counter);
        stdout().flush().unwrap();

        let guessed_code = share::prompt_for_code().unwrap();

        print!("\x1b[1A\x1b[0J{}\t\t", guessed_code.prettify());
        stdout().flush().unwrap();

        let code_match = actual_code.match_code(&guessed_code);
        guesses.push(solver::Guess((guessed_code, code_match)));

        thread::sleep(Duration::from_millis(300));
        for code_match in code_match.iter() {
            print!("{} ", code_match.prettify());
            stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(300));
        }

        if matches.is_present("show-possible") {
            show_possible_codes(&guesses);
        }

        guess = Some(guessed_code);
    }

    println!(
        "\n\n{} You needed {counter} tries",
        "Solved!".green().bold(),
    );
}

fn show_possible_codes(guesses: &[solver::Guess]) {
    print!(
        " \x1b[38;5;238m{}\x1b[0m",
        solver::possible_codes(guesses).len()
    );
}
