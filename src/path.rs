use std::ops::Add;

use convert_case::{Case, Casing};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TypePath {
    pub segments: Vec<String>,
}

impl TypePath {
    pub fn new() -> Self {
        Self {
            segments: vec![],
        }
    }

    pub fn parse<T: Into<String>>(s: T) -> Self {
        let s: String = s.into();
        let segments: Vec<_> = s.split(".").map(|v| v.to_case(Case::Pascal)).collect();

        Self { segments }
    }

    pub fn path(&self) -> String {
        self.segments.join(".")
    }
}

impl<T: Into<String>, const N: usize> From<[T; N]> for TypePath {
    fn from(segments: [T; N]) -> Self {
        Self {
            segments: segments.map(|v| v.into().to_case(Case::Pascal)).into(),
        }
    }
}

impl From<&'_ str> for TypePath {
    fn from(s: &'_ str) -> Self {
        Self::parse(s)
    }
}

impl From<&TypePath> for String {
    fn from(p: &TypePath) -> Self {
        p.path()
    }
}

impl Add<&'_ Self> for TypePath {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self.segments.extend_from_slice(&rhs.segments);

        self
    }
}

impl Add<&'_ str> for TypePath {
    type Output = Self;

    fn add(self, rhs: &str) -> Self::Output {
        self + &Self::from(rhs)
    }
}

impl std::fmt::Display for TypePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
