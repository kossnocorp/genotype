use std::path::PathBuf;

use genotype_lang_rs_config::{RSLangConfig, RSProjectConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GTConfigRS {
    pub enabled: Option<bool>,
    pub out: Option<PathBuf>,
    pub derive: Option<Vec<String>>,
    /// Rust package data
    pub package: Option<toml::Value>,
}

impl GTConfigRS {
    pub fn derive_project(_name: &Option<String>, config: &Option<GTConfigRS>) -> RSProjectConfig {
        let out = config
            .as_ref()
            .and_then(|c| c.out.clone())
            .unwrap_or_else(|| PathBuf::from("rs"));

        let mut lang = RSLangConfig::default();
        if let Some(derive) = &config.as_ref().and_then(|c| c.derive.clone()) {
            lang.derive = derive.clone();
        }

        let package = config.as_ref().and_then(|c| {
            c.package
                .as_ref()
                .and_then(|p| toml::to_string_pretty(&p).ok())
        });

        RSProjectConfig { out, lang, package }
    }
}
