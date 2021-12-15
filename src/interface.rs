use std::fmt::Display;

use convert_case::{Case, Casing};
use itertools::{Either, Itertools};

use super::SETTINGS;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Interface {
    pub name: String,
    pub extends: Option<String>,
    pub entries: Vec<InterfaceEntry>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct InterfaceEntry {
    pub key: String,
    pub value: String,
    pub optional: bool,
}

impl Interface {
    pub fn new(name: &str, extends: Option<&str>) -> Self {
        Self {
            name: name.to_case(Case::Pascal),
            entries: vec![],
            extends: extends.map(String::from),
        }
    }
}

impl Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indentation = &SETTINGS.read().indentation;
        let to_sort = SETTINGS.read().sort;

        if let Some(interface) = &self.extends {
            write!(
                f,
                "export interface {} extends {} {{\n",
                self.name, interface
            )?;
        } else {
            write!(f, "export interface {} {{\n", self.name)?;
        };

        let iter = if to_sort {
            Either::Left(
                self.entries
                    .iter()
                    .sorted_by_key(|e| format!("{}{}", e.optional, e.key)),
            )
        } else {
            Either::Right(self.entries.iter())
        };
        iter.map(|e| {
            if e.optional {
                write!(f, "{}{}?: {};\n", indentation, e.key, e.value)
            } else {
                write!(f, "{}{}: {};\n", indentation, e.key, e.value)
            }
        })
        .collect::<std::fmt::Result>()?;

        write!(f, "}}")
    }
}
