use clap::{Parser, Subcommand};
use commands::build::{build_command, GTBuildCommand};
use diagnostic::error::GTCliError;

pub mod commands;
pub mod diagnostic;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Weights a file
    Build(GTBuildCommand),
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build(args)) => build_command(args),

        None => Err(GTCliError::MissingCommand.into()),
    }
}
