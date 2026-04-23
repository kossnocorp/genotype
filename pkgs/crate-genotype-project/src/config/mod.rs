use crate::prelude::internal::*;

mod pkg;
pub use pkg::*;

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GtpConfig {
    /// Project name.
    pub name: Option<String>,
    /// Global package version used as default for enabled language manifests.
    pub version: Option<Version>,
    /// Project root directory relative to the cwd. It defaults to ".".
    #[serde(default = "GtpConfig::default_root")]
    pub root: RelativePathBuf,
    /// Dist directory relative to the root directory. It defaults to "dist".
    #[serde(default = "GtpConfig::default_dist")]
    #[serde(alias = "out")]
    pub dist: RelativePathBuf,
    /// Source directory relative to the root directory. It defaults to "src".
    #[serde(default = "GtpConfig::default_src")]
    pub src: RelativePathBuf,
    /// Project entry pattern. It defaults to `**/*.type` relative to [GtpConfig::src].
    #[serde(default = "GtpConfig::default_entry")]
    pub entry: RelativePathBuf,
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

    pub fn default_root() -> RelativePathBuf {
        ".".into()
    }

    pub fn default_dist() -> RelativePathBuf {
        "dist".into()
    }

    pub fn default_src() -> RelativePathBuf {
        "src".into()
    }

    pub fn default_entry() -> RelativePathBuf {
        "**/*.type".into()
    }
}

impl Default for GtpConfig {
    fn default() -> Self {
        GtpConfig {
            name: None,
            version: None,
            root: GtpConfig::default_root(),
            dist: GtpConfig::default_dist(),
            src: GtpConfig::default_src(),
            entry: GtpConfig::default_entry(),
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
