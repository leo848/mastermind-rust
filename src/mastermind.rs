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
    fn from_guess_string(string: &str) -> Option<Self> where Self: Sized;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Colors {
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

impl Colors {
    /// Return a char from a color.
    pub fn to_char(&self) -> char {
        match self {
            Colors::Blue => 'b',
            Colors::Red => 'r',
            Colors::Green => 'g',
            Colors::Yellow => 'y',
            Colors::Purple => 'p',
            Colors::White => 'w',
        }
    }

    /// Create a color from a character.
    pub fn from_char(character: char) -> Option<Self> {
        match character {
            'b' => Some(Colors::Blue),
            'r' => Some(Colors::Red),
            'g' => Some(Colors::Green),
            'y' => Some(Colors::Yellow),
            'p' => Some(Colors::Purple),
            'w' => Some(Colors::White),
            _ => None,
        }
    }

    pub fn all() -> [Colors; 6] {
        use Colors::*;
        [Blue, Red, Green, Yellow, Purple, White]
    }
}

impl Prettify for Colors {
    /// Prettify this color to a ANSI colored unicode character.
    fn prettify(&self) -> String {
        let symbol = "⬤";
        match self {
            Colors::Blue => symbol.blue(),
            Colors::Red => symbol.red(),
            Colors::Green => symbol.green(),
            Colors::Yellow => symbol.yellow(),
            Colors::Purple => symbol.truecolor(143, 0, 255),
            Colors::White => symbol.normal(),
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
    pub fn from_char(character: char) -> Option<Self> {
        match character {
            'n' => Some(MatchLevels::NoMatch),
            'c' => Some(MatchLevels::ColorMatch),
            'e' => Some(MatchLevels::ExactMatch),
            _ => None,
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
