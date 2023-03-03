use std::{io::Error, path::PathBuf};

use serde::Deserialize;

use crate::features::Feature;

#[derive(Deserialize)]
pub struct Config {
    pub source: Option<String>,
    pub target: String,
    pub features: Vec<Feature>,
    pub ignore: Option<Vec<String>>,
}

impl Config {
    pub fn read_from_file(path: PathBuf) -> Result<Config, Error> {
        let file = std::fs::read_to_string(path);
        if file.is_err() {
            return Err(file.err().unwrap());
        }
        let file = file.unwrap();
        match serde_json::from_str::<Config>(&file) {
            Ok(c) => Ok(c),
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }
}
