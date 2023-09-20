use std::{error::Error, fs::File, io::Read};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub root: String,
}

pub fn read_config_from_file(config_path: &str) -> Result<Config, Box<dyn Error>> {
    let mut file = File::open(config_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    match serde_yaml::from_str(&contents) {
        Ok(config) => Ok(config),
        Err(e) => Err(Box::new(e)),
    }
}
