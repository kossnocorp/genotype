use clap::{Parser, Subcommand};
use commands::{
    build::{build_command, GTBuildCommand},
    init::{init_command, GTInitCommand},
};
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
    /// Builds a Genotype project
    Build(GTBuildCommand),

    /// Initializes a Genotype project
    Init(GTInitCommand),
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build(args)) => build_command(args),

        Some(Commands::Init(args)) => init_command(args),

        None => Err(GTCliError::MissingCommand.into()),
    }
}
