pub mod code;
pub mod code_match;
pub mod share;
pub mod solver;

use colored::Colorize;
use itertools::Itertools;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;

pub trait Prettify {
    fn prettify(&self) -> String;
}

pub trait GuessString {
    fn to_guess_string(&self) -> String;
    fn from_guess_string(string: &str) -> Self;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Colors {
    Empty,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

impl Colors {
    /// Return a char from a color.
    ///
    /// # Panics
    /// If the color is empty.
    pub fn to_char(&self) -> char {
        match self {
            Colors::Blue => 'b',
            Colors::Red => 'r',
            Colors::Green => 'g',
            Colors::Yellow => 'y',
            Colors::Purple => 'p',
            Colors::White => 'w',
            Colors::Empty => panic!("to_char called on empty color"),
        }
    }

    /// Create a color from a character.
    ///
    /// # Panics
    /// If the character is not a valid color.
    pub fn from_char(character: char) -> Self {
        match character {
            'b' => Colors::Blue,
            'r' => Colors::Red,
            'g' => Colors::Green,
            'y' => Colors::Yellow,
            'p' => Colors::Purple,
            'w' => Colors::White,
            _ => panic!("unknown color char"),
        }
    }

    pub fn all() -> [Colors; 6] {
        use Colors::*;
        [Blue, Red, Green, Yellow, Purple, White]
    }
}

impl Prettify for Colors {
    /// Prettify this color to a ANSI colored unicode character.
    ///
    /// # Panics
    /// This method will panic if called on an empty color.
    fn prettify(&self) -> String {
        let symbol = "⬤";
        match self {
            Colors::Blue => symbol.blue(),
            Colors::Red => symbol.red(),
            Colors::Green => symbol.green(),
            Colors::Yellow => symbol.yellow(),
            Colors::Purple => symbol.truecolor(143, 0, 255),
            Colors::White => symbol.normal(),
            Colors::Empty => panic!("prettify called on empty color"),
        }
        .to_string()
    }
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Distribution<Colors> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Colors {
        match rng.gen_range(0..=5) {
            0 => Colors::Blue,
            1 => Colors::Red,
            2 => Colors::Green,
            3 => Colors::Yellow,
            4 => Colors::Purple,
            _ => Colors::White,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum MatchLevels {
    NoMatch,
    ColorMatch,
    ExactMatch,
}

impl MatchLevels {
    pub fn to_char(&self) -> char {
        match self {
            MatchLevels::NoMatch => 'n',
            MatchLevels::ColorMatch => 'c',
            MatchLevels::ExactMatch => 'e',
        }
    }
    /// Create a `MatchLevels` from a character.
    ///
    /// # Panics
    /// If the character is not a valid match level.
    pub fn from_char(character: char) -> Self {
        match character {
            'n' => MatchLevels::NoMatch,
            'c' => MatchLevels::ColorMatch,
            'e' => MatchLevels::ExactMatch,
            _ => panic!("unknown match character"),
        }
    }
    pub fn all() -> [MatchLevels; 3] {
        use MatchLevels::*;
        [ExactMatch, ColorMatch, NoMatch]
    }
}

impl Prettify for MatchLevels {
    fn prettify(&self) -> String {
        match self {
            MatchLevels::NoMatch => "◯".normal(),
            MatchLevels::ColorMatch => "⬤".normal(),
            MatchLevels::ExactMatch => "⬤".red(),
        }
        .to_string()
    }
}

impl fmt::Display for MatchLevels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
