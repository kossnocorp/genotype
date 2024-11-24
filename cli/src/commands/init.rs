use crate::diagnostic::error::GTCliError;
use clap::Args;
use genotype_config::{GTConfig, GTConfigPY, GTConfigRS, GTConfigTS};
use inquire::{
    list_option::ListOption, min_length, required, validator::Validation, MultiSelect, Text,
};
use miette::Result;

use heck::{ToKebabCase, ToSnakeCase};
use owo_colors::OwoColorize;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

#[derive(Args)]
pub struct GTInitCommand {
    /// Where to initialize the project, by default it will be the current
    /// directory.
    path: Option<PathBuf>,
}

pub fn init_command(args: &GTInitCommand) -> Result<()> {
    let mut config = GTConfig::default();

    let name = configure_name(&mut config)?;
    configure_targers(&mut config, &name)?;

    let root = args.path.clone().unwrap_or_else(|| PathBuf::from("."));

    create_dir_all(root.clone())
        .map_err(|_| GTCliError::FailedCreateDir(root.to_string_lossy().into()))?;

    write(
        root.join("genotype.toml"),
        toml::to_string(&config).map_err(|_| GTCliError::StringifyConfig)?,
    )
    .map_err(|_| GTCliError::FailedWrite("genotype.toml".into()))?;

    let src = root.join(config.src());

    create_dir_all(src.clone())
        .map_err(|_| GTCliError::FailedCreateDir(src.to_string_lossy().into()))?;

    for (file, content) in GUIDE_FILES {
        write(src.join(file), content)
            .map_err(|_| GTCliError::FailedWrite(src.join(file).to_string_lossy().into()))?;
    }

    println!(
        "{generated} project at {path:?}, run `{command}` to build the project",
        generated = "Generated".green().bold(),
        path = root.to_string_lossy(),
        command = "gt build".yellow().bold()
    );

    Ok(())
}

const GUIDE_FILES: &'static [(&str, &str)] = &[
    (
        "guide.type",
        include_str!("../../examples/guide/guide.type"),
    ),
    (
        "module.type",
        include_str!("../../examples/guide/module.type"),
    ),
];

fn configure_name(config: &mut GTConfig) -> Result<String> {
    let cd_name = std::env::current_dir()
        .map(|path| path.file_name().unwrap().to_string_lossy().to_string())
        .unwrap_or_default();

    let name = Text::new("Name your project:")
        .with_default(&cd_name)
        .with_validator(required!("Please provide the project name"))
        .with_validator(min_length!(1, "Please provide the project name"))
        .prompt()
        .map_err(|_| GTCliError::FailedReadline("project name"))?;

    config.name = Some(name.clone());

    Ok(name)
}

fn configure_targers(config: &mut GTConfig, name: &String) -> Result<()> {
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
            Target::TypeScript => configure_ts(config, &name)?,
            Target::Python => configure_py(config, &name)?,
            Target::Rust => configure_rs(config, &name)?,
        }
    }

    Ok(())
}

fn configure_ts(config: &mut GTConfig, name: &String) -> Result<()> {
    let mut ts = GTConfigTS::default();

    let default_name = name.to_kebab_case();
    let name = Text::new("Name the TypeScript package:")
        .with_default(&default_name)
        .with_validator(required!("Please provide the TypeScript package name"))
        .with_validator(min_length!(1, "Please provide the TypeScript package name"))
        .with_validator(|name: &str| {
            let re = Regex::new(r"^(@[a-z0-9][a-z0-9\-_\.]*\/)?[a-z0-9][a-z0-9\-_\.]*$")?;
            Ok(if re.is_match(name) {
                Validation::Valid
            } else {
                Validation::Invalid("Invalid package name".into())
            })
        })
        .prompt()
        .map_err(|_| GTCliError::FailedReadline("TypeScript package name"))?;

    let package = toml::Value::Table(toml::map::Map::from_iter(vec![
        ("name".into(), toml::Value::String(name.clone())),
        ("version".into(), toml::Value::String("0.1.0".into())),
    ]));

    ts.package = Some(package);

    config.ts = Some(ts);

    Ok(())
}

fn configure_py(config: &mut GTConfig, name: &String) -> Result<()> {
    let mut py = GTConfigPY::default();

    let default_name = name.to_kebab_case();
    let name = Text::new("Name the Python package:")
        .with_default(&default_name)
        .with_validator(required!("Please provide the Python package name"))
        .with_validator(min_length!(1, "Please provide the Python package name"))
        .with_validator(|name: &str| {
            let re = Regex::new(r"^[A-Za-z0-9][A-Za-z0-9_-]*$")?;
            Ok(if re.is_match(name) {
                Validation::Valid
            } else {
                Validation::Invalid("Invalid package name".into())
            })
        })
        .prompt()
        .map_err(|_| GTCliError::FailedReadline("Python package name"))?;

    let package = toml::Value::Table(toml::map::Map::from_iter(vec![
        ("name".into(), toml::Value::String(name.clone())),
        ("version".into(), toml::Value::String("0.1.0".into())),
    ]));

    py.package = Some(package);

    config.python = Some(py);

    Ok(())
}

fn configure_rs(config: &mut GTConfig, name: &String) -> Result<()> {
    let mut rs = GTConfigRS::default();

    let default_name = name.to_snake_case();
    let name = Text::new("Name the Rust crate:")
        .with_default(&default_name)
        .with_validator(required!("Please provide the Rust crate name"))
        .with_validator(min_length!(1, "Please provide the Rust crate name"))
        .with_validator(|name: &str| {
            let re = Regex::new(r"^[A-Za-z0-9][A-Za-z0-9_-]*$")?;
            Ok(if re.is_match(name) {
                Validation::Valid
            } else {
                Validation::Invalid("Invalid crate name".into())
            })
        })
        .prompt()
        .map_err(|_| GTCliError::FailedReadline("Rust package name"))?;

    let package = toml::Value::Table(toml::map::Map::from_iter(vec![
        ("name".into(), toml::Value::String(name.clone())),
        ("version".into(), toml::Value::String("0.1.0".into())),
        ("edition".into(), toml::Value::String("2021".into())),
    ]));

    rs.package = Some(package);

    config.rust = Some(rs);

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
