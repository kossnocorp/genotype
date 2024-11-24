use crate::diagnostic::error::GTCliError;
use clap::Args;
use genotype_config::GTConfig;
use genotype_lang_core_project::project::GTLangProject;
use genotype_lang_py_project::project::PYProject;
use genotype_lang_rs_project::project::RSProject;
use genotype_lang_ts_project::project::TSProject;
use genotype_project::GTProject;
use genotype_writer::GTWriter;
use miette::Result;
use owo_colors::OwoColorize;
use promkit::{
    preset::{checkbox::Checkbox, readline::Readline},
    suggest::Suggest,
};
use std::path::PathBuf;

#[derive(Args)]
pub struct GTInitCommand {
    /// Where to initialize the project, by default it will be the current
    /// directory.
    path: Option<PathBuf>,
}

pub fn init_command(args: &GTInitCommand) -> Result<()> {
    let targets = Checkbox::new(vec!["TypeScript", "Python", "Rust"])
        .title("Choose the languages you want to target")
        .checkbox_lines(5)
        .prompt()
        .map_err(|_| GTCliError::FailedReadline("targets"))?
        .run()
        .map_err(|_| GTCliError::FailedReadline("targets"))?;

    println!("Targets: {:?}", targets);

    Ok(())
}
