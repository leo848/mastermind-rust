use crate::mastermind::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeMatch(pub [MatchLevels; 4]);

impl Deref for CodeMatch {
    type Target = [MatchLevels; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CodeMatch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
