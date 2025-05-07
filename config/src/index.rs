use crate::prelude::internal::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GtConfig {
    /// Project name.
    pub name: Option<String>,
    /// Project root directory relative to the cwd. It defaults to ".".
    #[serde(default = "GtConfig::default_root")]
    pub root: GtRootPath,
    /// Dist directory relative to the root directory. It defaults to "dist".
    #[serde(default = "GtConfig::default_dist")]
    pub out: GtRootRelativePath,
    /// Source directory relative to the root directory. It defaults to "src".
    #[serde(default = "GtConfig::default_src")]
    pub src: GtRootRelativePath,
    /// Project entry pattern. It defaults to `**/*.type` relative to the project's source
    /// directory.
    #[serde(default = "GtConfig::default_entry")]
    pub entry: GtSrcRelativePath,
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
    pub fn ts_enabled(&self) -> bool {
        self.ts.common.enabled
    }

    pub fn python_enabled(&self) -> bool {
        self.py.common.enabled
    }

    pub fn rust_enabled(&self) -> bool {
        self.rs.common.enabled
    }
    pub fn from_root(name: &str, root: &str) -> Self {
        GtConfig {
            name: Some(name.into()),
            root: GtRootPath::new(root.into()),
            src: ".".into(),
            ..GtConfig::default()
        }
    }

    pub fn from_entry(name: &str, root: &str, entry: &str) -> Self {
        GtConfig {
            name: Some(name.into()),
            root: GtRootPath::new(root.into()),
            entry: entry.into(),
            src: ".".into(),
            ..GtConfig::default()
        }
    }

    pub fn default_root() -> GtRootPath {
        GtRootPath::new(".".into())
    }

    pub fn default_dist() -> GtRootRelativePath {
        "dist".into()
    }

    pub fn default_src() -> GtRootRelativePath {
        "src".into()
    }

    pub fn default_entry() -> GtSrcRelativePath {
        "**/*.type".into()
    }
}

impl Default for GtConfig {
    fn default() -> Self {
        GtConfig {
            name: None,
            root: GtConfig::default_root(),
            out: GtConfig::default_dist(),
            src: GtConfig::default_src(),
            entry: GtConfig::default_entry(),
            ts: TsConfig::default(),
            py: PyConfig::default(),
            rs: RsConfig::default(),
        }
    }
}
