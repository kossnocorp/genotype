use std::path::PathBuf;

use genotype_lang_py_config::{PYConfig, PYVersion};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GTConfig {
    /// The root directory
    pub root: PathBuf,
    /// The entry pattern
    entry: PathBuf,
    /// The out directory
    out: PathBuf,

    /// Python config
    python: Option<PYConfig>,
}

impl GTConfig {
    pub fn from_root(root: &str) -> Self {
        GTConfig {
            root: root.into(),
            ..GTConfig::default()
        }
    }

    pub fn from_entry(root: &str, entry: &str) -> Self {
        GTConfig {
            root: root.into(),
            entry: entry.into(),
            ..GTConfig::default()
        }
    }

    pub fn out(&self) -> PathBuf {
        if self.out.is_absolute() {
            self.out.clone()
        } else {
            self.root.join(&self.out)
        }
    }

    pub fn entry_pattern(&self) -> String {
        if self.entry.is_absolute() {
            self.entry.clone()
        } else {
            self.root.join(&self.entry)
        }
        .to_str()
        // [TODO] Handle error
        .unwrap()
        .to_owned()
    }

    pub fn python_version(&self) -> PYVersion {
        self.python
            .as_ref()
            .and_then(|p| p.version.clone())
            .unwrap_or_default()
    }

    pub fn with_python(&mut self, config: PYConfig) -> &mut Self {
        self.python = Some(config);
        self
    }
}

impl Default for GTConfig {
    fn default() -> GTConfig {
        GTConfig {
            entry: "**/*.type".into(),
            root: "./".into(),
            out: "./out".into(),

            python: None,
        }
    }
}
