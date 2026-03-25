use clap::{Parser, Subcommand};
use commands::{
    build::{GtBuildCommand, build_command},
    init::{GtInitCommand, init_command},
    version::{GtVersionCommand, version_command},
};
use diagnostic::error::GtCliError;

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
    Build(GtBuildCommand),

    /// Initializes a Genotype project
    Init(GtInitCommand),

    /// Manage package versions in genotype.toml
    Version(GtVersionCommand),
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build(args)) => build_command(args),

        Some(Commands::Init(args)) => init_command(args),

        Some(Commands::Version(args)) => version_command(args),

        None => Err(GtCliError::MissingCommand.into()),
    }
}
