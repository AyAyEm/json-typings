use std::{error::Error, fs, path::Path};

use clap::{App, Arg};
use json_typings::{read_json, strategy::Strategy, validators, Settings, Typing, SETTINGS};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Json typings")
        .version("1.0.0")
        .author("André Azevedo Meira <AyAyEm.dev@gmail.com>")
        .about("Converts JSON files into typescript typing declarations")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
                .validator_os(validators::file_exists),
        )
        .args(&[
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .value_name("STRING")
                .help("Defines the start and the end of strings"),
            Arg::with_name("indentation")
                .short("i")
                .long("indentation")
                .value_name("STRING")
                .help("Defines how many tabs or spaces to insert inside a scope"),
            Arg::with_name("typescript_version")
                .short("t")
                .long("typescript_version")
                .value_name("SEMVER")
                .help(
                    "Specify the typescript version to automatically disable incompatible features",
                ),
            Arg::with_name("sort")
                .long("sort")
                .help("Enable sorting of interface keys"),
            Arg::with_name("tree")
                .long("tree")
                .help("Sets the formating strategy to tree"),
            Arg::with_name("family")
                .long("family")
                .help("Sets the formating strategy to family"),
        ])
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets the output target file")
                .default_value("index.d.ts"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1)
                .validator_os(validators::file_exists),
        )
        .get_matches();

    if let Some(v) = matches.value_of("config") {
        *SETTINGS.write() = Settings::from_config(v)?;
    }

    if let Some(v) = matches.value_of("delimiter") {
        SETTINGS.write().string_delimiter = String::from(v);
    }

    if let Some(v) = matches.value_of("indentation") {
        SETTINGS.write().indentation = String::from(v);
    }

    if let Some(v) = matches.value_of("typescript_version") {
        SETTINGS.write().typescript_version = String::from(v);
    }

    if matches.is_present("sort") {
        SETTINGS.write().sort = true;
    }

    if matches.is_present("family") {
        SETTINGS.write().strategy = Strategy::Family;
    }

    if matches.is_present("tree") {
        SETTINGS.write().strategy = Strategy::Tree;
    }

    let input = matches.value_of("INPUT").unwrap();
    let value = read_json::file(Path::new(input))?;

    let typings = match &SETTINGS.read().strategy {
        Strategy::Tree => {
            vec![Typing::from_items("All", value)]
        }
        Strategy::Family => todo!(),
    };

    let output = matches.value_of("output").unwrap();
    let output = Path::new(output);
    let output = output.with_extension("ts");
    if let Some(p) = output.parent() {
        fs::create_dir_all(p)?;
    };

    let typings = typings
        .iter()
        .map(|t| t.as_string(Strategy::Tree))
        .fold(String::new(), |acc, c| acc + &c);

    fs::write(output, format!("{}\n", typings))?;

    Ok(())
}
