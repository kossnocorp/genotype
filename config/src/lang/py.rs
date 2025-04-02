use std::{collections::HashMap, path::PathBuf};

use genotype_lang_py_config::{PYLangConfig, PYProjectConfig, PYVersion};
use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};

use crate::{error::GTConfigError, result::GTConfigResult};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GTConfigPY {
    pub enabled: Option<bool>,
    /// Path where to generate the Python package. It defaults to `py` relative to the project's
    /// out directory.
    pub out: Option<PathBuf>,
    /// Python version to use. It defaults to the latest stable version.
    pub version: Option<PYVersion>,
    /// Python module name. If not provided the project name will be used.
    pub module: Option<String>,
    /// pyproject.toml data.
    pub package: Option<toml::Value>,
    /// Manually mapped dependencies.
    pub dependencies: Option<HashMap<String, String>>,
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
        out: PathBuf,
        config: &Option<GTConfigPY>,
    ) -> GTConfigResult<PYProjectConfig> {
        Ok(PYProjectConfig {
            out: out.join(
                config
                    .as_ref()
                    .and_then(|c| c.out.clone())
                    .unwrap_or_else(|| PathBuf::from("py")),
            ),
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
            dependencies: config
                .as_ref()
                .and_then(|c| Some(c.dependencies.clone()))
                .unwrap_or_default(),
        })
    }
}

impl Default for GTConfigPY {
    fn default() -> Self {
        GTConfigPY {
            enabled: Some(true),
            out: Some(PathBuf::from("py")),
            version: Some(PYVersion::default()),
            module: None,
            package: None,
            dependencies: None,
        }
    }
}
