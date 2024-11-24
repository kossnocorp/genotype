use std::path::PathBuf;

use genotype_lang_rs_config::{RSLangConfig, RSProjectConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GTConfigRS {
    pub enabled: Option<bool>,
    /// Path where to generate the Rust package. It defaults to `rs` relative to the project's
    /// out directory.
    pub out: Option<PathBuf>,
    /// The traits to derive for the generated Rust structs and enums.
    pub derive: Option<Vec<String>>,
    /// Cargo.toml data.
    pub package: Option<toml::Value>,
}

impl GTConfigRS {
    pub fn derive_project(
        _name: &Option<String>,
        out: PathBuf,
        config: &Option<GTConfigRS>,
    ) -> RSProjectConfig {
        let out = out.join(
            config
                .as_ref()
                .and_then(|c| c.out.clone())
                .unwrap_or_else(|| PathBuf::from("rs")),
        );

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

impl Default for GTConfigRS {
    fn default() -> Self {
        GTConfigRS {
            enabled: Some(true),
            out: Some(PathBuf::from("rs")),
            derive: Some(RSLangConfig::default_derive()),
            package: None,
        }
    }
}
