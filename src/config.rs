use std::{collections::HashMap, error::Error, fs::File, io::Read};

use serde::Deserialize;

pub type ConfigHeaders = Vec<HashMap<String, String>>;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Host {
    pub host_name: String,
    pub root: String,
    pub add_header: Option<Vec<HashMap<String, String>>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub port: u16,
    pub root: String,
    pub hosts: Vec<Host>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_config_from_file() {
        let expected_config = Config {
            port: 3001,
            root: ".".to_string(),
            hosts: vec![
                Host {
                    host_name: "madoka.local".to_string(),
                    root: ".".to_string(),
                    add_header: Some(vec![HashMap::from([(
                        "madoka-debug".to_string(),
                        "1".to_string(),
                    )])]),
                },
                Host {
                    host_name: "blog.madoka.local".to_string(),
                    root: "./blog.madoka.local".to_string(),
                    add_header: Some(vec![
                        HashMap::from([("origin-trial".to_string(), "aaaaaaa".to_string())]),
                        HashMap::from([("origin-trial".to_string(), "bbbbbbb".to_string())]),
                    ]),
                },
            ],
        };
        let config = read_config_from_file("playground/madoka.conf.yaml").unwrap();
        assert_eq!(config, expected_config);
    }
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
