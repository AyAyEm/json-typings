use std::ops::{Add, Sub};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct NestingLevel(pub usize);

impl NestingLevel {
    pub fn to_string(&self, to_repeat: &str) -> String {
        to_repeat.repeat(self.0)
    }
}

impl Add for NestingLevel {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for NestingLevel {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<usize> for NestingLevel {
    fn from(s: usize) -> Self {
        Self(s)
    }
}
