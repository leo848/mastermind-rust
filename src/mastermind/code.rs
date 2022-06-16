use std::ops::{Deref, DerefMut};

use crate::mastermind::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code(pub [Colors; 4]);

impl Deref for Code {
    type Target = [Colors; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Code {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Code {
    pub fn match_code(&self, other: &Code) -> code_match::CodeMatch {
        let mut matches = [MatchLevels::NoMatch; 4];
        for i in 0..matches.len() {
            if self[i] == other[i] {
                matches[i] = MatchLevels::ExactMatch;
            } else if other.contains(&self[i]) {
                matches[i] = MatchLevels::ColorMatch;
            }
        }
        matches.sort();
        matches.reverse();
        code_match::CodeMatch(matches)
    }

    /// Trivial method that checks for duplicate colors.
    pub fn is_unique(&self) -> bool {
        self.iter().unique().count() == self.len()
    }

    /// Return a random code.
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut values: [Colors; 4] = [Colors::Empty; 4];

        for i in 0..4 {
            let mut new_value: Colors = rng.gen();
            while values.contains(&new_value) {
                new_value = rng.gen();
            }
            values[i] = new_value;
        }

        Code(values)
    }

    /// Returns all possible combinations of codes.
    /// Useful for a brute-force algorithm.
    pub fn all() -> Vec<Code> {
        let mut all = Vec::new();

        for c1 in Colors::all() {
            for c2 in Colors::all() {
                for c3 in Colors::all() {
                    for c4 in Colors::all() {
                        let code = Code([c1, c2, c3, c4]);
                        if code.is_unique() {
                            all.push(code);
                        }
                    }
                }
            }
        }

        all
    }
}
