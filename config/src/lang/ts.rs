use std::path::PathBuf;

use genotype_lang_ts_config::TSProjectConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GTConfigTS {
    pub enabled: Option<bool>,
    pub out: Option<PathBuf>,
    pub src: Option<PathBuf>,
}

impl Default for GTConfigTS {
    fn default() -> Self {
        GTConfigTS {
            enabled: None,
            out: None,
            src: None,
        }
    }
}

impl Into<TSProjectConfig> for GTConfigTS {
    fn into(self) -> TSProjectConfig {
        TSProjectConfig {
            out: self.out.unwrap_or(PathBuf::from("ts")),
            src: self.src.unwrap_or(PathBuf::from("src")),
        }
    }
}
