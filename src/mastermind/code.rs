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
}

