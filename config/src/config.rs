use std::path::PathBuf;

use genotype_lang_py_config::PYProjectConfig;
use genotype_lang_ts_config::TSProjectConfig;
use serde::{Deserialize, Serialize};

use crate::{error::GTConfigError, result::GTConfigResult, GTConfigPY, GTConfigTS};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GTConfig {
    /// Project name
    name: Option<String>,
    /// The root directory
    pub root: Option<PathBuf>,
    /// The source directory
    src: Option<PathBuf>,
    /// The entry pattern
    entry: Option<PathBuf>,
    /// The out directory
    out: Option<PathBuf>,
    /// TypeScript
    ts: Option<GTConfigTS>,
    /// Python config
    python: Option<GTConfigPY>,
}

impl GTConfig {
    pub fn out(&self) -> PathBuf {
        if let Some(out) = &self.out {
            if out.is_absolute() {
                out.clone()
            } else {
                self.root().join(&out)
            }
        } else {
            self.root().join("libs")
        }
    }

    pub fn root(&self) -> PathBuf {
        self.root.clone().unwrap_or(".".into())
    }

    pub fn src(&self) -> PathBuf {
        self.src.clone().unwrap_or("src".into())
    }

    pub fn entry(&self) -> PathBuf {
        self.entry.clone().unwrap_or("**/*.type".into())
    }

    pub fn entry_pattern(&self) -> GTConfigResult<String> {
        let entry = self.entry();

        Ok(if entry.is_absolute() {
            entry
        } else {
            self.root().join(self.src()).join(entry)
        }
        .to_str()
        .ok_or(GTConfigError::FailedToConstructEntry)?
        .to_owned())
    }

    pub fn ts_enabled(&self) -> bool {
        self.ts
            .as_ref()
            .map_or(false, |config| config.enabled.unwrap_or(false))
    }

    pub fn as_ts_project(&self) -> TSProjectConfig {
        GTConfigTS::derive_project(&self.name, &self.ts)
    }

    pub fn python_enabled(&self) -> bool {
        self.python
            .as_ref()
            .map_or(false, |config| config.enabled.unwrap_or(false))
    }

    pub fn as_python_project(&self) -> GTConfigResult<PYProjectConfig> {
        GTConfigPY::derive_project(&self.name, &self.python)
    }

    pub fn source_path(&self, path: &PathBuf) -> PathBuf {
        self.out().join(path)
    }

    pub fn from_root(name: &str, root: &str) -> Self {
        GTConfig {
            name: Some(name.into()),
            root: Some(root.into()),
            src: Some(".".into()),
            ..GTConfig::default()
        }
    }

    pub fn from_entry(name: &str, root: &str, entry: &str) -> Self {
        GTConfig {
            name: Some(name.into()),
            root: Some(root.into()),
            entry: Some(entry.into()),
            src: Some(".".into()),
            ..GTConfig::default()
        }
    }
}

impl Default for GTConfig {
    fn default() -> GTConfig {
        GTConfig {
            name: None,
            entry: None,
            root: None,
            src: None,
            out: None,
            ts: None,
            python: None,
        }
    }
}
