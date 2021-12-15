use std::{
    ffi::{OsStr, OsString},
    path::Path,
};

pub fn file_exists(v: &OsStr) -> Result<(), OsString> {
    if Path::new(v).exists() {
        Ok(())
    } else {
        Err(OsString::from("The passed file path does not exists"))
    }
}
