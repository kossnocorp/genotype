use crate::diagnostic::error::GTCliError;
use clap::Args;
use genotype_config::GTConfig;
use genotype_lang_core_project::project::GTLangProject;
use genotype_lang_py_project::project::PYProject;
use genotype_lang_ts_project::project::TSProject;
use genotype_project::GTProject;
use genotype_writer::GTWriter;
use miette::Result;
use owo_colors::OwoColorize;
use std::path::PathBuf;

#[derive(Args)]
pub struct GTBuildCommand {
    /// What to build
    #[arg(default_value = ".")]
    path: PathBuf,
}

pub fn build_command(args: &GTBuildCommand) -> Result<()> {
    let config = GTConfig::load(&args.path)?;

    let project = GTProject::load(&config)?;
    let ts = TSProject::generate(&project, &config)
        .map_err(|_| GTCliError::Generate)?
        .render(&config)
        .map_err(|_| GTCliError::Render)?;
    let py = PYProject::generate(&project, &config)
        .map_err(|_| GTCliError::Generate)?
        .render(&config)
        .map_err(|_| GTCliError::Render)?;
    GTWriter::new(vec![ts, py])
        .write()
        .map_err(|_| GTCliError::Write)?;

    println!(
        "{} project to {:?}",
        "Generated".green().bold(),
        config.out()
    );

    Ok(())
}
