use crate::diagnostic::error::GTCliError;
use clap::Args;
use genotype_config::GtConfig;
use genotype_lang_core_project::*;
use genotype_lang_py_project::*;
use genotype_lang_rs_project::*;
use genotype_lang_ts_project::*;
use genotype_project::GtProject;
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
    let config = GtConfig::load(&args.path)?;
    let project = GtProject::load(&config)?;

    let mut langs = vec![];

    if project.config.ts_enabled() {
        let ts = TsProject::generate(&project)?.dist()?;
        langs.push(ts);
    }

    if project.config.python_enabled() {
        let py = PyProject::generate(&project)?.dist()?;
        langs.push(py);
    }

    if project.config.rust_enabled() {
        let rs = RsProject::generate(&project)?.dist()?;
        langs.push(rs);
    }

    GTWriter::write(&langs, &project.config).map_err(|_| GTCliError::Write)?;

    println!(
        "{} project to {:?}",
        "Generated".green().bold(),
        project.config.dist_path()
    );

    Ok(())
}
