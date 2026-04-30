use crate::config;
use crate::executor;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub command: String,
}

pub fn cli() {
    load_commands();
}

fn load_commands() {
    let cli: Cli = Cli::parse();

    let config = config::load_config();

    match config.commands.get(&cli.command) {
        Some(path) => {
            if let Err(e) = executor::run_script(path) {
                eprintln!("Execution failed: {}", e);
            }
        }
        None => {
            eprintln!("Command not found");
        }
    }
}
