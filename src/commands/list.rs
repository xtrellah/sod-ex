use crate::config;

pub fn list() {
    let config = config::load_config();

    let toml = toml::to_string(&config).unwrap();

    println!("{}", toml);
}
