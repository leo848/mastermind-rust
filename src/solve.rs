use mastermind::mastermind::*;
use mastermind::{self, Code, CodeMatch};

use clap::ArgMatches;
use colored::Colorize;
use itertools::Itertools;
use std::{error, io};

pub fn run(matches: &ArgMatches) {
    println!("{}", "Solve mode\n".bold());

    if !matches.is_present("no-info") {
        print_info();
    }

    let mut possible_codes = Code::all();
    let mut counter = 0;

    loop {
        counter += 1;

        println!();
        println!("{} possible combinations.", possible_codes.len());
        println!("Enter guess #{}: ", counter);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: Vec<&str> = input.split_whitespace().collect();
        if input.len() != 2 {
            eprintln!(
                "Invalid input: {}!",
                if input.len() > 2 {
                    "too many elements"
                } else {
                    "not enough elements"
                }
            );
            counter -= 1;
            continue;
        }

        let guess: solver::Guess = match parse_input(input.try_into().unwrap()) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        println!("{}", guess.prettify());

        possible_codes = possible_codes
            .into_iter()
            .filter(|code| code.match_code(&guess.0 .0) == guess.0 .1)
            .collect();
    }
}

fn print_info() {
    println!("The following colors are available:");
    for color in Colors::all() {
        println!("{}: {} ({})", color.prettify(), color, color.to_char());
    }
    println!();

    println!("The following match levels are available:");
    for match_level in MatchLevels::all() {
        println!(
            "{}: {} ({})",
            match_level.prettify(),
            match_level,
            match_level.to_char()
        );
    }
    println!();

    println!("You can input a guess in the following form:");
    println!("<4 colors> <matches>\n");
    println!("Examples:");
    for _ in 0..5 {
        let random_code = code::Code::random();
        let code_match = code::Code::random().match_code(&random_code);
        println!(
            "{} {}",
            random_code.to_guess_string(),
            code_match.to_guess_string()
        );
    }
}

fn parse_input(input: [&str; 2]) -> Result<solver::Guess, Box<dyn error::Error>> {
    let (tested_code, res_guess) = input.into_iter().next_tuple().unwrap();

    Ok(solver::Guess((
        Code::from_guess_string(tested_code),
        CodeMatch::from_guess_string(res_guess),
    )))
}
