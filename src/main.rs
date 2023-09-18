use serde::Deserialize;
use std::fs::File;
use std::io::Read;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "madoka.conf.yaml")]
    config: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
}

fn main() {
    let args = Args::parse();
    let config_path = args.config;

    let mut file = match File::open(config_path) {
        Ok(file) => file,
        Err(e) => panic!("Error opening config file: {}", e),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => panic!("Error reading config file: {}", e),
    };

    let config: Config = match serde_yaml::from_str(&mut contents) {
        Ok(config) => config,
        Err(e) => panic!("Error parsing config file: {}", e),
    };

    let port = config.port;

    println!("Listening on port {}", port);
}
