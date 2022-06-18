pub mod code;
pub mod code_match;
pub mod solver;

use itertools::Itertools;
use colored::Colorize;
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

impl Prettify for Colors {
    fn prettify(&self) -> String {
        let symbol = "●";
        match self {
            Colors::Blue => symbol.blue(),
            Colors::Red => symbol.red(),
            Colors::Green => symbol.green(),
            Colors::Yellow => symbol.yellow(),
            Colors::Violet => symbol.truecolor(143, 0, 255),
            Colors::White => symbol.normal(),
            _ => panic!("Prettify called on empty color"),
        }.to_string()
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
