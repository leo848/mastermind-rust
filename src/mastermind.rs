pub mod code;
pub mod code_match;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Colors {
    Blue = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Violet = 4,
    White = 5,
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum MatchLevels {
    NoMatch,
    ColorMatch,
    ExactMatch,
}

