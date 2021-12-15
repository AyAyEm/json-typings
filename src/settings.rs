use std::{borrow::Borrow, fmt::Debug, hash::Hash};

use config::{Config, ConfigError, Environment, File, FileFormat};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::strategy::Strategy;

lazy_static! {
    pub static ref SETTINGS: RwLock<Settings> = RwLock::new(Settings::new());
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Settings {
    pub string_delimiter: String,
    pub indentation: String,
    pub typescript_version: String,
    pub strategy: Strategy,
    pub sort: bool,
}

#[allow(dead_code)]
impl Settings {
    pub fn new() -> Self {
        Self {
            string_delimiter: String::from("\""),
            indentation: String::from("    "),
            typescript_version: String::from("latest"),
            strategy: Strategy::Tree,
            sort: false,
        }
    }

    pub fn merge<T>(&self, b: T) -> Result<Self, ConfigError>
    where
        T: Borrow<Self>,
    {
        let to_merge_settings = [self, b.borrow()]
            .map(serde_json::to_string)
            .map(Result::unwrap);

        let mut settings = Config::default();
        for setting in to_merge_settings {
            settings.merge(File::from_str(&setting, FileFormat::Json))?;
        }

        settings.try_into()
    }

    pub fn from_env() -> Result<Self, ConfigError> {
        let default_settings = serde_json::to_string(&Self::new()).unwrap();
        let mut settings = Config::default();
        settings.merge(File::from_str(&default_settings, FileFormat::Json))?;
        settings.merge(Environment::new())?;

        settings.try_into()
    }

    pub fn from_config<T>(name: T) -> Result<Self, ConfigError>
    where
        T: AsRef<str>,
    {
        let default_settings = serde_json::to_string(&Self::new()).unwrap();
        let mut settings = Config::default();
        settings.merge(File::from_str(&default_settings, FileFormat::Json))?;
        settings.merge(File::with_name(name.as_ref()))?;

        settings.try_into()
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn merge() {
        let a = Settings::new();

        let mut b = Settings::new();
        b.string_delimiter = String::from("'");
        b.indentation = String::from("  ");

        let expected = Settings {
            string_delimiter: String::from("'"),
            indentation: String::from("  "),
            typescript_version: String::from("latest"),
            strategy: Strategy::Tree,
            sort: false,
        };
        assert_eq!(expected, a.merge(b).unwrap())
    }

    #[test]
    fn from_env() {
        let mut expected = Settings::new();
        expected.string_delimiter = String::from("'");

        env::set_var("STRING_DELIMITER", "'");
        assert_eq!(expected, Settings::from_env().unwrap())
    }

    #[test]
    fn from_config() {
        let mut exepected = Settings::new();
        exepected.indentation = String::from("  ");
        assert_eq!(
            exepected,
            Settings::from_config("config/test.toml").unwrap(),
        );
    }
}
