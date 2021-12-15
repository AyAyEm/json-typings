use std::{
    fmt::{self, Display},
    hash::{Hash, Hasher},
    mem::discriminant,
};

use ordered_float::OrderedFloat;
use serde_json::Number;

use crate::SETTINGS;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    String(String),
    Number(Number),
    Template(String),
}

impl Literal {
    pub fn from_str(s: &str) -> Self {
        Self::String(String::from(s))
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sep = &SETTINGS.try_read().ok_or(fmt::Error)?.string_delimiter;
        match self {
            Self::Number(n) => n.fmt(f),
            Self::Template(s) => write!(f, "`{}`", s),
            Self::String(s) => write!(f, "{0}{1}{0}", sep, s),
        }
    }
}

impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
        match self {
            Self::String(s) | Self::Template(s) => s.hash(state),
            Self::Number(n) => {
                if n.is_i64() {
                    n.as_i64().unwrap().hash(state);
                } else {
                    OrderedFloat(n.as_f64().unwrap()).hash(state);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_integer() {
        assert_eq!("1024", format!("{}", Literal::Number(Number::from(1024))));
        assert_eq!("-1024", format!("{}", Literal::Number(Number::from(-1024))));

        assert_eq!("0", format!("{}", Literal::Number(Number::from(0))));
        assert_eq!("0", format!("{}", Literal::Number(Number::from(-0))));
    }

    #[test]
    fn display_float() {
        assert_eq!(
            "7.65",
            format!("{}", Literal::Number(Number::from_f64(7.65).unwrap()))
        );
        assert_eq!(
            "-7.65",
            format!("{}", Literal::Number(Number::from_f64(-7.65).unwrap()))
        );

        assert_eq!(
            "0",
            format!("{}", Literal::Number(Number::from_f64(0.0).unwrap()))
        );
        assert_eq!(
            "-0",
            format!("{}", Literal::Number(Number::from_f64(-0.0).unwrap()))
        );
    }

    #[test]
    fn display_string() {
        assert_eq!(
            "\"test\"",
            format!("{}", Literal::String(String::from("test")))
        );
    }
}
