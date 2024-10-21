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

    let mut langs = vec![];

    if config.ts_enabled() {
        let ts_config = config.as_ts_project();
        let ts = TSProject::generate(&project, &ts_config)
            .map_err(|_| GTCliError::Generate)?
            .render(&ts_config)
            .map_err(|_| GTCliError::Render)?;
        langs.push(ts);
    }

    if config.python_enabled() {
        let py_config = config.as_python_project().unwrap();
        let py = PYProject::generate(&project, &py_config)
            .map_err(|_| GTCliError::Generate)?
            .render(&py_config)
            .map_err(|_| GTCliError::Render)?;
        langs.push(py);
    }

    GTWriter::write(&langs, &config).map_err(|_| GTCliError::Write)?;

    println!(
        "{} project to {:?}",
        "Generated".green().bold(),
        config.out()
    );

    Ok(())
}
