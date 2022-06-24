use crate::mastermind::*;

use std::ops::{Deref, DerefMut};

/// A guess contains a Code and a corresponding CodeMatch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guess(pub (code::Code, code_match::CodeMatch));

impl Prettify for Guess {
    fn prettify(&self) -> String {
        format!("{}\t\t{}", self.0 .0.prettify(), self.0 .1.prettify())
    }
}

impl Deref for Guess {
    type Target = (code::Code, code_match::CodeMatch);
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Guess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Get the possible codes from any number of guesses
///
/// This is done by taking all codes and filtering them by matching them against every guess.
pub fn possible_codes(guesses: &[Guess]) -> Vec<code::Code> {
    code::Code::all()
        .into_iter()
        .filter(|code| {
            guesses
                .iter()
                .all(|guess| (guess.0 .0).match_code(code) == (guess.0 .1))
        })
        .collect()
}

/// Get the best possible guess, i.e., the guess after which the least amount of codes are
/// possible.
///
/// This algorithm is optimized for the worst case
pub fn best_guess(guesses: &[Guess]) -> (usize, code::Code) {
    let possible_codes_before = possible_codes(guesses);

    let mut best_guess = (usize::MAX, code::Code::empty());

    // Every code is guessable.
    for guessable_code in code::Code::all() {
        println!("testing {}\x1b[1A", guessable_code.prettify());

        let max_possible_codes = possible_codes_before
            .iter()
            .map(|possible_code| {
                possible_codes_before
                    .iter()
                    .filter(|code| {
                        possible_code.match_code(code) == possible_code.match_code(&guessable_code)
                    })
                    .count()
            })
            .max()
            .unwrap();

        // // A possible code has the possibility of being the true code.
        // for possible_code in &possible_codes_before {
        //     // let guess = Guess((guessable_code, possible_code.match_code(&guessable_code)));

        //     let possible_codes_after = possible_codes_before
        //         .iter()
        //         .filter(|code| {
        //             possible_code.match_code(code) == possible_code.match_code(&guessable_code)
        //         })
        //         .count();
        //     max_possible_codes = usize::max(max_possible_codes, possible_codes_after)
        // }
        if max_possible_codes < best_guess.0 {
            best_guess = (max_possible_codes, guessable_code)
        }
    }

    best_guess
}
