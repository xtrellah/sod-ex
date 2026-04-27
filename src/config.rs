use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub commands: HashMap<String, String>,
}

pub fn load_config() -> Config {
    let content = fs::read_to_string("config/example.toml").expect("Failed to read config file");

    toml::from_str(&content).expect("Failed to parse config")
}
