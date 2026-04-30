use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub commands: HashMap<String, String>,
}

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().expect("Could not find config directory");

    path.push("sod-ex");
    path.push("config.toml");

    path
}

pub fn ensure_config_dir() {
    let mut dir = dirs::config_dir().expect("Could not find config directory");

    dir.push("sod-ex");

    fs::create_dir_all(&dir).expect("Failed to create config directory");
}

pub fn ensure_config_file(path: &std::path::Path) {
    if !path.exists() {
        let default = r#"
[commands]
example = "/path/to/script.sh"
"#;

        fs::write(path, default).expect("Failed to create config file");
    }
}

pub fn load_config() -> Config {
    let path = get_config_path();

    ensure_config_dir();

    ensure_config_file(&path);

    let content = fs::read_to_string(&path).expect("Failed to read config file");

    toml::from_str(&content).expect("Failed to parse config")
}

pub fn add_entry(name: &str, script_path: &str) -> Config {
    let path = get_config_path();

    let mut config: Config = load_config();

    // add entry
    config
        .commands
        .insert(name.to_string(), script_path.to_string());

    // write
    let updated = toml::to_string_pretty(&config).expect("Failed to serialize config");

    fs::write(&path, updated).expect("Failed to write config file");

    config
}
