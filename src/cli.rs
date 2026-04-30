use crate::commands;
use crate::config;
use crate::executor;
// use clap::Parser;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Lists aliased scripts")]
    List {},
}

impl Commands {
    fn as_str(&self) -> &str {
        match self {
            Commands::List {} => "list",
        }
    }
}

pub fn cli() {
    let cli: Cli = Cli::parse();

    load_commands(&cli);

    match cli.command {
        Commands::List {} => {
            commands::list::list();
        }
    }
}

fn load_commands(cli: &Cli) {
    let config = config::load_config();

    match config.commands.get(cli.command.as_str()) {
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
