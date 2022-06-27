use mastermind::mastermind::*;
use mastermind::{self, Code, CodeMatch};

use clap::ArgMatches;
use colored::Colorize;
use itertools::Itertools;
use std::{
    error,
    io::{self, Write},
};

pub fn run(matches: &ArgMatches) {
    println!("{}", "Solve mode\n".bold());

    if !matches.is_present("no-info") {
        share::print_info();
    }

    let mut possible_codes = Code::all();
    let mut guess_history: Vec<solver::Guess> = Vec::new();

    let mut counter = 0;

    while possible_codes.len() > 1 {
        counter += 1;

        println!();
        println!("{} possible combinations.", possible_codes.len());

        let code = solver::best_guess(&guess_history);
        println!("Best guess: {}", code.prettify(),);

        print!("Enter guess #{}: ", counter);
        io::stdout().flush().unwrap();

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

        guess_history.push(guess);
    }

    println!();
    if possible_codes.len() == 1 {
        println!("{}", "Solution found!".bold().green());
        println!("{}", possible_codes[0].prettify());
    } else {
        println!("{}", "Didn't find a solution - invalid input?".bold().red());
    }
}

fn parse_input(input: [&str; 2]) -> Result<solver::Guess, Box<dyn error::Error>> {
    let (tested_code, res_guess) = input.into_iter().next_tuple().unwrap();

    Ok(solver::Guess((
        Code::from_guess_string(tested_code),
        CodeMatch::from_guess_string(res_guess),
    )))
}
