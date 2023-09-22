use std::{error::Error, fs::File, io::Read};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Host {
    pub host_name: String,
    pub root: String,
}

#[derive(Debug, Deserialize)]
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
        let config = read_config_from_file("playground/madoka.conf.yaml").unwrap();
        assert_eq!(config.port, 3000);
        assert_eq!(config.root, ".");
        assert_eq!(config.hosts.len(), 2);
        assert_eq!(config.hosts[0].host_name, "madoka.local");
        assert_eq!(config.hosts[0].root, ".");
        assert_eq!(config.hosts[1].host_name, "blog.madoka.local");
        assert_eq!(config.hosts[1].root, "./blog.madoka.local");
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
