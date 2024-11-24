use crate::diagnostic::error::GTCliError;
use clap::Args;
use genotype_config::{GTConfig, GTConfigPY, GTConfigRS, GTConfigTS};
use inquire::{
    list_option::ListOption, min_length, required, validator::Validation, MultiSelect, Text,
};
use miette::Result;

use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Args)]
pub struct GTInitCommand {
    /// Where to initialize the project, by default it will be the current
    /// directory.
    path: Option<PathBuf>,
}

pub fn init_command(args: &GTInitCommand) -> Result<()> {
    let mut config = GTConfig::default();

    // Name

    let name = Text::new("Name your project:")
        .with_validator(required!("Please provide the project name"))
        .with_validator(min_length!(1, "Please provide the project name"))
        .prompt()
        .map_err(|_| GTCliError::FailedReadline("name"))?;

    config.name = Some(name);

    // Targets

    let targets = MultiSelect::new(
        "Choose the languages you want to target:",
        Target::VARIANTS.to_vec(),
    )
    .with_validator(|targets: &[ListOption<&Target>]| {
        Ok(if targets.len() < 1 {
            Validation::Invalid("Please select at least one language".into())
        } else {
            Validation::Valid
        })
    })
    .prompt()
    .map_err(|_| GTCliError::FailedReadline("targets"))?;

    for target in targets {
        match target {
            Target::TypeScript => {
                let ts = GTConfigTS::default();
                // [TODO] Generate name
                // [TODO] Generate package data
                config.ts = Some(ts);
            }

            Target::Python => {
                let py = GTConfigPY::default();
                // [TODO] Generate name
                // [TODO] Generate package data
                config.python = Some(py);
            }

            Target::Rust => {
                let rs = GTConfigRS::default();
                // [TODO] Generate name
                // [TODO] Generate package data
                config.rust = Some(rs);
            }
        }
    }

    // [TODO] Write config to genotype.toml

    // [TODO] Create src directory

    // [TODO] Create guide file

    // [TODO] Suggest to run `genotype build` after the project is initialized

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Target {
    TypeScript,
    Python,
    Rust,
}

impl Target {
    const VARIANTS: &'static [Target] = &[Self::TypeScript, Self::Python, Self::Rust];
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }
}
