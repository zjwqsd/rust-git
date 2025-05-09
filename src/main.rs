mod cli;
mod commands;
mod core;
mod utils;
use cli::args::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    cli.execute();
}
