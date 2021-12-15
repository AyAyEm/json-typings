use std::fmt::Display;

use convert_case::{Case, Casing};
use itertools::Itertools;

use crate::utils::add_indentation;

use super::{Interface, SETTINGS};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Namespace {
    pub name: String,
    pub interface: Interface,
    pub entries: Vec<NamespaceEntry>,
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct NamespaceEntry {
    pub key: String,
    pub value: NamespaceEntryValue,
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub enum NamespaceEntryValue {
    Alias(String),
    Namespace(String),
}

impl Namespace {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_case(Case::Pascal),
            interface: Interface::new(name, None),
            entries: vec![],
        }
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indentation = &SETTINGS.read().indentation;

        self.interface.fmt(f)?;

        if self.entries.len() > 0 {
            write!(f, "\n\nexport namespace {} {{\n", self.name)?;
            self.entries
                .iter()
                .sorted()
                .map(|e| {
                    let formated_value = match &e.value {
                        NamespaceEntryValue::Alias(a) => {
                            format!("export type {} = {};\n", e.key, a)
                        }
                        NamespaceEntryValue::Namespace(n) => format!("{}\n", n),
                    };

                    add_indentation(indentation, &formated_value)
                })
                .join("\n").fmt(f)?;

            write!(f, "}}")
        } else {
            Ok(())
        }
    }
}

impl NamespaceEntry {
    pub fn new(key: &str, value: NamespaceEntryValue) -> Self {
        Self {
            key: String::from(key),
            value,
        }
    }
}
