use crate::commands;
use crate::config;
use crate::executor;
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
    #[command(about = "Add entry")]
    Add { name: String, path: String },
    #[command(external_subcommand)]
    External(Vec<String>),
}

impl Commands {
    fn as_str(&self) -> &str {
        match self {
            Commands::List {} => "list",
            Commands::Add { name, path } => "add",
            Commands::External(args) => args.first().map(|s| s.as_str()).unwrap_or(""),
        }
    }
}

pub fn cli() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::List {} => {
            commands::list::list();
        }
        Commands::Add { name, path } => {
            commands::add::add(&name, &path);
        }
        Commands::External(args) => {
            if let Some(cmd_name) = args.first() {
                let config = config::load_config();
                if let Some(path) = config.commands.get(cmd_name) {
                    let script_args = &args[1..];
                    if let Err(e) = executor::run_script_with_args(path, script_args) {
                        eprintln!("Execution failed: {}", e);
                    }
                } else {
                    eprintln!("Command '{}' not found", cmd_name);
                }
            }
        }
    }
}
