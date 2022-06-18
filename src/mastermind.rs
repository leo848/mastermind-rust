pub mod code;
pub mod code_match;
pub mod solver;

use std::fmt;
use colored::Colorize;
use itertools::Itertools;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub trait Prettify {
    fn prettify(&self) -> String;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Colors {
    Empty,
    Blue,
    Red,
    Green,
    Yellow,
    Violet,
    White,
}

impl Colors {
    pub fn to_char(&self) -> char {
        match self {
            Colors::Blue => 'b',
            Colors::Red => 'r',
            Colors::Green => 'g',
            Colors::Yellow => 'y',
            Colors::Violet => 'v',
            Colors::White => 'w',
            _ => panic!("to_char called on empty color")
        }
    }
}

impl Prettify for Colors {
    fn prettify(&self) -> String {
        let symbol = "⬤";
        match self {
            Colors::Blue => symbol.blue(),
            Colors::Red => symbol.red(),
            Colors::Green => symbol.green(),
            Colors::Yellow => symbol.yellow(),
            Colors::Violet => symbol.truecolor(143, 0, 255),
            Colors::White => symbol.normal(),
            _ => panic!("prettify called on empty color"),
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
            4 => Colors::Violet,
            _ => Colors::White,
        }
    }
}

impl Colors {
    pub fn all() -> [Colors; 6] {
        use Colors::*;
        [Blue, Red, Green, Yellow, Violet, White]
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum MatchLevels {
    NoMatch,
    ColorMatch,
    ExactMatch,
}
