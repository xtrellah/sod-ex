use crate::cli::cli;

mod cli;
mod commands;
mod config;
mod executor;

fn main() {
    cli();
}
