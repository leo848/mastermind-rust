use std::ops::{Deref, DerefMut};

use crate::mastermind::*;

/// A code is a wrapper for four colors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code(pub [Colors; 4]);

impl Code {
    /// Match this code with another one and get a CodeMatch.
    ///
    /// ```
    /// use mastermind::mastermind::code::*;
    /// use mastermind::mastermind::code_match::*;
    /// use mastermind::mastermind::MatchLevels::*;
    /// use mastermind::mastermind::Colors::*;
    ///
    /// let hidden_code = Code([ Yellow, Red, White, Green ]);
    /// let guess = Code([ Red, Yellow, Violet, Green ]);
    ///
    /// let code_match = guess.match_code(&hidden_code);
    /// assert_eq!(code_match, CodeMatch([ ExactMatch, ColorMatch, ColorMatch, NoMatch ]));
    /// ```
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
    ///
    /// ```
    /// use mastermind::mastermind::Colors::*;
    /// use mastermind::mastermind::code::*;
    ///
    /// let unique_code = Code([ Yellow, Green, Blue, White ]);
    /// assert!(unique_code.is_unique());
    ///
    /// let repeating_code = Code([ Green, Green, Violet, Red ]);
    /// assert!(!repeating_code.is_unique());
    /// ```
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
    ///
    /// ```
    /// use mastermind::mastermind::code::*;
    ///
    /// let all_codes = Code::all();
    /// assert_eq!(all_codes.len(), 6*5*4*3);
    /// ```
    pub fn all() -> Vec<Self> {
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

    pub fn empty() -> Self {
        use Colors::*;
        Code([ Empty, Empty, Empty, Empty ])
    }
}

impl GuessString for Code {
    fn to_guess_string(&self) -> String {
        self.iter().map(|color| color.to_char()).collect()
    }
    fn from_guess_string(string: &str) -> Self {
        assert_eq!(string.len(), 4, "Wrong string length");
        Code(
            string
                .chars()
                .map(Colors::from_char)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

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

impl Prettify for Code {
    fn prettify(&self) -> String {
        itertools::Itertools::intersperse(self.iter().map(|c| c.prettify()), " ".to_string())
            .collect()
    }
}
