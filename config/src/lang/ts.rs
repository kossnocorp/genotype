use std::{collections::HashMap, path::PathBuf};

use genotype_lang_ts_config::TSProjectConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GTConfigTS {
    pub enabled: Option<bool>,
    /// Path where to generate the TypeScript package. It defaults to `./ts` relative to
    /// the project's out directory.
    pub out: Option<PathBuf>,
    /// Source directory where the TypeScript files are located. It defaults to `./src` relative to
    /// the out directory.
    pub src: Option<PathBuf>,
    /// package.json data.
    pub package: Option<toml::Value>,
    /// Manually mapped dependencies.
    pub dependencies: Option<HashMap<String, String>>,
}

impl GTConfigTS {
    pub fn derive_project(
        _name: &Option<String>,
        out: PathBuf,
        config: &Option<GTConfigTS>,
    ) -> TSProjectConfig {
        TSProjectConfig {
            out: out.join(
                config
                    .as_ref()
                    .and_then(|c| c.out.clone())
                    .unwrap_or_else(|| PathBuf::from("ts")),
            ),
            src: config
                .as_ref()
                .and_then(|c| c.src.clone())
                .unwrap_or("src".into()),
            package: config.as_ref().and_then(|c| c.package.clone()),
            dependencies: config
                .as_ref()
                .and_then(|c| Some(c.dependencies.clone()))
                .unwrap_or_default(),
        }
    }
}

impl Default for GTConfigTS {
    fn default() -> Self {
        GTConfigTS {
            enabled: Some(true),
            out: Some(PathBuf::from("ts")),
            src: Some("src".into()),
            package: None,
            dependencies: Default::default(),
        }
    }
}
