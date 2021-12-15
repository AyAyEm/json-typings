use std::{
    error::Error,
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use ijson::IValue;
use serde_json::{self, Value};

#[allow(dead_code)]
pub fn file(path: &Path) -> Result<IValue, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);

    serde_json::from_reader(reader).or_else(|err| Err(Box::new(err) as Box<dyn std::error::Error>))
}

#[allow(dead_code)]
pub fn dir(folder: &Path) -> Result<Vec<Value>, Box<dyn Error>> {
    fs::read_dir(folder)?
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| match entry.path().extension() {
            Some(e) => e == "json",
            None => false,
        })
        .map(|json_entry| json_entry.path())
        .map(File::open)
        .filter_map(|file| file.ok())
        .map(BufReader::new)
        .map(|json_reader| {
            let result = serde_json::from_reader(json_reader);
            result.or_else(|err| Err(Box::new(err) as Box<dyn std::error::Error>))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn file() {
        let value = super::file(&Path::new("data/sample_a.json"));

        dbg!(&value);
    }
}
