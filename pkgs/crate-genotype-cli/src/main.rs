use prelude::internal::*;

mod commands;
pub use commands::*;

mod diagnostic;
pub use diagnostic::*;

pub mod prelude;

#[derive(Parser)]
#[command(name = "gt", version, about, long_about = None)]
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

        None => {
            let mut command = Cli::command();
            command.print_help().into_diagnostic()?;
            println!();
            Ok(())
        }
    }
}
