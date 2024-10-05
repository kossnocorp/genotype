use clap::{Parser, Subcommand};
use genotype_lang_core_project::project::GTLangProject;
use genotype_lang_ts_project::project::TSProject;
use genotype_project::GTProject;
use genotype_writer::GTWriter;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Weights a file
    Build {
        /// The entry points
        #[arg(short, long, default_value = "*.type")]
        entry: String,
        /// The root directory
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
        /// The out directory
        #[arg(short, long, default_value = "./out")]
        out: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build { entry, root, out }) => {
            let root = root.canonicalize()?;
            let out = root.join(out);

            // [TODO] Use PathBuf instead of &str
            let project = GTProject::load(root.as_os_str().to_str().unwrap(), entry)?;
            let ts = TSProject::generate(&project, out.as_os_str().to_str().unwrap())?.render()?;
            GTWriter::new(vec![ts]).write()?;

            Ok(())
        }
        None => Ok(()),
    }
}
