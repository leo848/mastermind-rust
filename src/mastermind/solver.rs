use crate::mastermind::*;

use std::ops::{Deref, DerefMut};

/// A guess contains a `Code` and a corresponding `CodeMatch`.
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
/// This algorithm is a stub and just returns the first possible guess at this moment.
pub fn best_guess(guesses: &[Guess]) -> code::Code {
    let possible_true_codes = possible_codes(guesses);

    possible_true_codes[0]
}
