use genotype_lang_py_config::*;
use genotype_lang_rs_config::*;
use genotype_lang_ts_config::*;
use miette::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::error::GtConfigError;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GtConfig {
    /// Project name.
    pub name: Option<String>,
    /// Project root directory used to resolve relative paths. It defaults to the current directory.
    #[serde(default = "GtConfig::default_root")]
    pub root: PathBuf,
    /// Project out directory. It defaults to `./libs` relative to the project's root directory.
    #[serde(default = "GtConfig::default_out")]
    pub out: PathBuf,
    /// Where the Genotype source files are located. It defaults to `./src` relative to
    /// the project's root directory.
    #[serde(default = "GtConfig::default_src")]
    pub src: PathBuf,
    /// Project entry pattern. It defaults to `**/*.type` relative to the project's source
    /// directory.
    #[serde(default = "GtConfig::default_entry")]
    pub entry: PathBuf,
    /// TypeScript config.
    #[serde(default)]
    pub ts: TsConfig,
    /// Python config.
    #[serde(default)]
    pub py: PyConfig,
    /// Rust config.
    #[serde(default)]
    pub rs: RsConfig,
}

impl GtConfig {
    pub fn full_out(&self) -> PathBuf {
        if self.out.is_absolute() {
            self.out.clone()
        } else {
            self.root.join(&self.out)
        }
    }

    pub fn full_entry(&self) -> Result<String> {
        Ok(if self.entry.is_absolute() {
            self.entry.clone()
        } else {
            self.root.join(&self.src).join(&self.entry)
        }
        .to_str()
        .ok_or(GtConfigError::FailedToConstructEntry)?
        .to_owned())
    }

    pub fn ts_enabled(&self) -> bool {
        self.ts.common.enabled
    }

    pub fn python_enabled(&self) -> bool {
        self.py.common.enabled
    }

    pub fn rust_enabled(&self) -> bool {
        self.rs.common.enabled
    }

    pub fn file_path(&self, path: &PathBuf) -> PathBuf {
        self.root.join(path)
    }

    pub fn from_root(name: &str, root: &str) -> Self {
        GtConfig {
            name: Some(name.into()),
            root: root.into(),
            src: ".".into(),
            ..GtConfig::default()
        }
    }

    pub fn from_entry(name: &str, root: &str, entry: &str) -> Self {
        GtConfig {
            name: Some(name.into()),
            root: root.into(),
            entry: entry.into(),
            src: ".".into(),
            ..GtConfig::default()
        }
    }

    pub fn default_root() -> PathBuf {
        PathBuf::from(".")
    }

    pub fn default_out() -> PathBuf {
        PathBuf::from("libs")
    }

    pub fn default_src() -> PathBuf {
        PathBuf::from("src")
    }

    pub fn default_entry() -> PathBuf {
        PathBuf::from("**/*.type")
    }
}

impl Default for GtConfig {
    fn default() -> Self {
        GtConfig {
            name: None,
            root: GtConfig::default_root(),
            out: GtConfig::default_out(),
            src: GtConfig::default_src(),
            entry: GtConfig::default_entry(),
            ts: TsConfig::default(),
            py: PyConfig::default(),
            rs: RsConfig::default(),
        }
    }
}
