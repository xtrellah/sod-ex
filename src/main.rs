mod cli;
mod config;
mod executor;

use clap::Parser;

use config::load_config;

fn main() {
    let cli = cli::Cli::parse();

    let config = load_config();

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
