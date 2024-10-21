use std::path::PathBuf;

use genotype_lang_ts_config::TSProjectConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GTConfigTS {
    pub enabled: Option<bool>,
    pub out: Option<PathBuf>,
    pub src: Option<PathBuf>,
    /// TypeScript package data
    pub package: Option<toml::Value>,
}

impl Default for GTConfigTS {
    fn default() -> Self {
        GTConfigTS {
            enabled: None,
            out: None,
            src: None,
            package: None,
        }
    }
}

impl GTConfigTS {
    pub fn derive_project(_name: &Option<String>, config: &Option<GTConfigTS>) -> TSProjectConfig {
        TSProjectConfig {
            out: config
                .as_ref()
                .and_then(|c| c.out.clone())
                .unwrap_or_else(|| PathBuf::from("ts")),
            src: config
                .as_ref()
                .and_then(|c| c.src.clone())
                .unwrap_or("src".into()),
            package: config.as_ref().and_then(|c| c.package.clone()),
        }
    }
}
