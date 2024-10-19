use crate::diagnostic::error::GTCliError;
use clap::Args;
use genotype_lang_core_project::project::GTLangProject;
use genotype_lang_py_project::project::PYProject;
use genotype_lang_py_tree::{PYOptions, PYVersion};
use genotype_lang_ts_project::project::TSProject;
use genotype_project::GTProject;
use genotype_writer::GTWriter;
use miette::Result;
use owo_colors::OwoColorize;
use std::path::PathBuf;

#[derive(Args)]
pub struct GTBuildCommand {
    /// The entry points
    #[arg(short, long, default_value = "**/*.type")]
    entry: String,
    /// The root directory
    #[arg(short, long, default_value = ".")]
    root: PathBuf,
    /// The out directory
    #[arg(short, long, default_value = "./out")]
    out: PathBuf,
}

pub fn build_command(args: &GTBuildCommand) -> Result<()> {
    let root = args
        .root
        .canonicalize()
        .map_err(|_| GTCliError::Canonicalize(format!("root directory {:?}", args.root)))?;

    let out = root.join(args.out.clone());

    // [TODO] Use PathBuf instead of &str
    let project = GTProject::load(root.as_os_str().to_str().unwrap(), &args.entry)?;
    let ts = TSProject::generate(&project, out.as_os_str().to_str().unwrap())
        .map_err(|_| GTCliError::Generate)?
        .render(&())
        .map_err(|_| GTCliError::Render)?;
    let py = PYProject::generate(&project, out.as_os_str().to_str().unwrap())
        .map_err(|_| GTCliError::Generate)?
        .render(&PYOptions::new(PYVersion::Legacy))
        .map_err(|_| GTCliError::Render)?;
    GTWriter::new(vec![ts, py])
        .write()
        .map_err(|_| GTCliError::Write)?;

    println!("{} project to {:?}", "Generated".green().bold(), out);

    Ok(())
}
