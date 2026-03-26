use crate::diagnostic::error::GtCliError;
use clap::Args;
use genotype_config::GtConfig;
use genotype_lang_core_config::*;
use genotype_lang_core_project::*;
use genotype_lang_py_project::*;
use genotype_lang_rs_project::*;
use genotype_lang_ts_project::*;
use genotype_path::GtRelativePath;
use genotype_project::GtProject;
use miette::Result;
use owo_colors::OwoColorize;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

#[derive(Args)]
pub struct GtBuildCommand {
    /// What to build
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Path to genotype config file
    #[arg(long)]
    config: Option<PathBuf>,
}

pub fn build_command(args: &GtBuildCommand) -> Result<()> {
    let config = GtConfig::load_with(&args.path, args.config.as_ref())?;
    let project = GtProject::load(&config)?;

    let mut langs = vec![];

    if project.config.ts_enabled() {
        print_notices(&project.config.ts.health_check());
        let ts = TsProject::generate(&project)?.dist()?;
        langs.push(ts);
    }

    if project.config.python_enabled() {
        print_notices(&project.config.py.health_check());
        let py = PyProject::generate(&project)?.dist()?;
        langs.push(py);
    }

    if project.config.rust_enabled() {
        print_notices(&project.config.rs.health_check());
        let rs = RsProject::generate(&project)?.dist()?;
        langs.push(rs);
    }

    write_dist(&langs).map_err(|_| GtCliError::Write)?;

    println!(
        "{} project to {:?}",
        "Generated".green().bold(),
        project.config.dist_path()
    );

    Ok(())
}

fn write_dist(projects: &Vec<GtlProjectDist>) -> Result<(), Box<dyn std::error::Error>> {
    for project in projects {
        project
            .files
            .iter()
            .map(|module| {
                let dir = module.path.relative_path().parent().unwrap();
                create_dir_all(dir.to_path(""))?;
                write(module.path.relative_path().to_path(""), &module.source)
            })
            .collect::<Result<(), _>>()?;
    }

    Ok(())
}

fn print_notices(notices: &Vec<GtlConfigNotice>) {
    for notice in notices {
        print_notice(notice);
    }
}

fn print_notice(notice: &GtlConfigNotice) {
    match notice.kind {
        GtlConfigNoticeKind::Warning => {
            eprintln!(
                "{label}: {message}",
                label = "Warning".yellow().bold(),
                message = notice.message
            );
        }

        GtlConfigNoticeKind::Info => {
            println!(
                "{label}: {message}",
                label = "Info".blue().bold(),
                message = notice.message
            );
        }
    }
}
