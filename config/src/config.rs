use std::path::PathBuf;

use genotype_lang_py_config::PYProjectConfig;
use genotype_lang_rs_config::RSProjectConfig;
use genotype_lang_ts_config::TSProjectConfig;
use serde::{Deserialize, Serialize};

use crate::{error::GTConfigError, result::GTConfigResult, GTConfigPY, GTConfigRS, GTConfigTS};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GTConfig {
    /// Project name. If not provided it will be inferred from the root directory.
    pub name: Option<String>,
    /// Project root directory used to resolve relative paths. It defaults to the current directory.
    pub root: Option<PathBuf>,
    /// Project out directory. It defaults to `./libs` relative to the project's root directory.
    pub out: Option<PathBuf>,
    /// Where the Genotype source files are located. It defaults to `./src` relative to
    /// the project's root directory.
    pub src: Option<PathBuf>,
    /// Project entry pattern. It defaults to `**/*.type` relative to the project's source
    /// directory.
    pub entry: Option<PathBuf>,
    /// TypeScript config.
    pub ts: Option<GTConfigTS>,
    /// Python config.
    pub python: Option<GTConfigPY>,
    /// Rust config.
    pub rust: Option<GTConfigRS>,
}

impl GTConfig {
    pub fn full_out(&self) -> PathBuf {
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

    pub fn out(&self) -> PathBuf {
        if let Some(out) = &self.out {
            out.clone()
        } else {
            PathBuf::from("libs")
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
        GTConfigTS::derive_project(&self.name, self.out(), &self.ts)
    }

    pub fn python_enabled(&self) -> bool {
        self.python
            .as_ref()
            .map_or(false, |config| config.enabled.unwrap_or(false))
    }

    pub fn as_python_project(&self) -> GTConfigResult<PYProjectConfig> {
        GTConfigPY::derive_project(&self.name, self.out(), &self.python)
    }

    pub fn rust_enabled(&self) -> bool {
        self.rust
            .as_ref()
            .map_or(false, |config| config.enabled.unwrap_or(false))
    }

    pub fn as_rust_project(&self) -> RSProjectConfig {
        GTConfigRS::derive_project(&self.name, self.out(), &self.rust)
    }

    pub fn source_path(&self, path: &PathBuf) -> PathBuf {
        self.root().join(path)
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

    pub fn with_rust(&mut self, config: GTConfigRS) {
        self.rust = Some(config);
    }
}

impl Default for GTConfig {
    fn default() -> GTConfig {
        GTConfig {
            name: None,
            root: None,
            out: Some(PathBuf::from("libs")),
            src: Some(PathBuf::from("src")),
            entry: Some(PathBuf::from("**/*.type")),
            ts: None,
            python: None,
            rust: None,
        }
    }
}
