mod varint;
mod ping;
mod rcon;

use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub rcon_host: String,
    pub password: String
}

fn read_config() -> Config {
    let mut file = File::open("config.json").expect("Failed to open config.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config.json");

    serde_json::from_str(&contents).expect("Failed to parse config.json")
}