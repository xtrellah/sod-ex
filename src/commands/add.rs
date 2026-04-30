use crate::config;

pub fn add(name: &str, path: &str) {
    config::add_entry(name, path);

    println!("{} = {}", name, path);
}
