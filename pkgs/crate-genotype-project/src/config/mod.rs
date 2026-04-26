use crate::prelude::internal::*;

mod error;
pub use error::*;

mod lang;
pub use lang::*;

mod load;

mod save;

mod paths;

mod toml_str;

mod manifest;
pub use manifest::*;

pub const GTCONFIG_FILE: &str = "genotype.toml";

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GtpConfig {
    /// Project name.
    pub name: Option<String>,
    /// Global package version used as default for enabled language manifests.
    pub version: Option<Version>,
    /// Project root directory relative to the cwd. It defaults to ".".
    pub root: GtpConfigDirRelativeRootDirPath,
    /// Dist directory relative to the root directory. It defaults to "dist".
    #[serde(alias = "out")]
    pub dist: GtpRootDirRelativeDistDirPath,
    /// Source directory relative to the root directory. It defaults to "src".
    pub src: GtpRootDirRelativeSrcDirPath,
    /// Project entry pattern. It defaults to `**/*.type` relative to [GtpConfig::src].
    pub entry: GtpSrcDirRelativeEntryPattern,
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

impl GtpConfig {
    pub fn parse(source: String) -> Result<Self> {
        let config: GtpConfig = Self::from_toml_str(&source)?;
        Ok(config)
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

    pub fn from_root(name: &str, root: &str) -> Self {
        GtpConfig {
            name: Some(name.into()),
            version: None,
            root: root.into(),
            src: ".".into(),
            source_toml_str: String::new(),
            ..GtpConfig::default()
        }
    }

    pub fn from_entry(name: &str, root: &str, entry: &str) -> Self {
        GtpConfig {
            name: Some(name.into()),
            version: None,
            root: root.into(),
            entry: entry.into(),
            src: ".".into(),
            source_toml_str: String::new(),
            ..GtpConfig::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_global_version() {
        let config = toml::from_str::<GtpConfig>("version = \"0.2.0\"\n").unwrap();
        assert_eq!(config.version, Some(Version::parse("0.2.0").unwrap()));
    }

    #[test]
    fn test_parse_language_full_aliases() {
        let config = toml::from_str::<GtpConfig>(
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
        let config = toml::from_str::<GtpConfig>(
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
