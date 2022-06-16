use crate::mastermind::*;

use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guess(pub (code::Code, code_match::CodeMatch));

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

pub struct Solver;

impl Solver {
    pub fn possible_codes(guesses: &[Guess]) -> Vec<code::Code> {
        code::Code::all()
            .into_iter()
            .filter(|code| {
                guesses
                    .iter()
                    .all(|guess| (guess.0.0).match_code(code) == (guess.0.1))
            })
            .collect()
    }
}
