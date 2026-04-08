use crate::prelude::internal::*;

mod load;

mod save;

mod paths;

mod toml_str;

mod manifest;
pub use manifest::*;

pub const GTCONFIG_FILE: &str = "genotype.toml";

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GtConfig {
    /// Project name.
    pub name: Option<String>,
    /// Global package version used as default for enabled language manifests.
    pub version: Option<Version>,
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
    pub entry: GtEntryPath,
    /// TypeScript config.
    #[serde(default, alias = "typescript")]
    pub ts: TsConfig,
    /// Python config.
    #[serde(default, alias = "python")]
    pub py: PyConfig,
    /// Rust config.
    #[serde(default, alias = "rust")]
    pub rs: RsConfig,
    #[serde(skip)]
    source_toml_str: String,
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
            version: None,
            root: GtRootPath::new(root.into()),
            src: ".".into(),
            source_toml_str: String::new(),
            ..GtConfig::default()
        }
    }

    pub fn from_entry(name: &str, root: &str, entry: &str) -> Self {
        GtConfig {
            name: Some(name.into()),
            version: None,
            root: GtRootPath::new(root.into()),
            entry: GtEntryPath::new(entry.into()),
            src: ".".into(),
            source_toml_str: String::new(),
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

    pub fn default_entry() -> GtEntryPath {
        GtEntryPath::new("**/*.type".into())
    }
}

impl Default for GtConfig {
    fn default() -> Self {
        GtConfig {
            name: None,
            version: None,
            root: GtConfig::default_root(),
            out: GtConfig::default_dist(),
            src: GtConfig::default_src(),
            entry: GtConfig::default_entry(),
            ts: TsConfig::default(),
            py: PyConfig::default(),
            rs: RsConfig::default(),
            source_toml_str: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_global_version() {
        let config = toml::from_str::<GtConfig>("version = \"0.2.0\"\n").unwrap();
        assert_eq!(config.version, Some(Version::parse("0.2.0").unwrap()));
    }

    #[test]
    fn test_parse_language_full_aliases() {
        let config = toml::from_str::<GtConfig>(
            r#"[python]
enabled = true
version = "latest"

[rust]
enabled = true
"#,
        )
        .unwrap();

        assert!(config.python_enabled());
        assert!(config.rust_enabled());
    }

    #[test]
    fn test_parse_language_short_aliases() {
        let config = toml::from_str::<GtConfig>(
            r#"[py]
enabled = true
version = "latest"

[rs]
enabled = true
"#,
        )
        .unwrap();

        assert!(config.python_enabled());
        assert!(config.rust_enabled());
    }
}
