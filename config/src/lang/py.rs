use std::path::PathBuf;

use genotype_lang_py_config::{PYLangConfig, PYProjectConfig, PYVersion};
use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};

use crate::{error::GTConfigError, result::GTConfigResult};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GTConfigPY {
    pub enabled: Option<bool>,
    pub out: Option<PathBuf>,
    pub version: Option<PYVersion>,
    /// Python module name. If not provided the project name will be used.
    pub module: Option<String>,
    /// Python package data
    pub package: Option<toml::Value>,
}

impl GTConfigPY {
    pub fn derive_module(
        name: &Option<String>,
        config: &Option<GTConfigPY>,
    ) -> GTConfigResult<String> {
        match (name, config.as_ref().and_then(|c| c.module.clone())) {
            (_, Some(module)) => Ok(module),
            (Some(name), _) => Ok(name.to_snake_case()),
            _ => Err(GTConfigError::PythonMissingModuleName),
        }
    }

    pub fn derive_project(
        name: &Option<String>,
        config: &Option<GTConfigPY>,
    ) -> GTConfigResult<PYProjectConfig> {
        Ok(PYProjectConfig {
            out: config
                .as_ref()
                .and_then(|c| c.out.clone())
                .unwrap_or_else(|| PathBuf::from("py")),
            module: GTConfigPY::derive_module(name, config)?,
            lang: PYLangConfig {
                version: config
                    .as_ref()
                    .and_then(|c| c.version.clone())
                    .unwrap_or_default(),
            },
            package: config.as_ref().and_then(|c| {
                c.package
                    .as_ref()
                    .and_then(|p| toml::to_string_pretty(&p).ok())
            }),
        })
    }
}
