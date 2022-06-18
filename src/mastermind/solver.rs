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
pub fn possible_codes(guesses: &[Guess]) -> Vec<code::Code> {
    // Return all codes
    code::Code::all()
        .into_iter()
        // where for every code
        .filter(|code| {
            guesses
                .iter()
                // every guess matches
                .all(|guess| (guess.0 .0).match_code(code) == (guess.0 .1))
        })
        .collect()
}
