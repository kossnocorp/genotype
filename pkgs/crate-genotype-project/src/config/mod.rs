use crate::prelude::internal::*;

mod error;
pub use error::*;

mod lang;
pub use lang::*;

mod save;

mod toml_str;

mod manifest;
pub use manifest::*;

pub const GTCONFIG_FILE: &str = "genotype.toml";

const fn default_package() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GtpConfig {
    /// Project name.
    pub name: Option<String>,
    /// Global package version used as default for enabled language manifests.
    pub version: Option<Version>,
    /// Whether to generate package structure and metadata by default for all targets.
    #[serde(default = "default_package")]
    pub package: bool,
    /// Project root directory relative to the cwd. It defaults to ".".
    #[serde(default)]
    pub root: GtpConfigDirRelativeRootDirPath,
    /// Dist directory relative to the root directory. It defaults to "dist".
    #[serde(default)]
    pub dist: GtpRootDirRelativeDistDirPath,
    /// Source directory relative to the root directory. It defaults to "src".
    #[serde(default)]
    pub src: GtpRootDirRelativeSrcDirPath,
    /// Project entry pattern. It defaults to `**/*.type` relative to [GtpConfig::src].
    #[serde(default)]
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

impl Default for GtpConfig {
    fn default() -> Self {
        Self {
            name: None,
            version: None,
            package: true,
            root: Default::default(),
            dist: Default::default(),
            src: Default::default(),
            entry: Default::default(),
            ts: Default::default(),
            py: Default::default(),
            rs: Default::default(),
            source_toml_str: String::new(),
        }
    }
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

    #[test]
    fn test_package_global() {
        let config = toml::from_str::<GtpConfig>("name = \"demo\"\n").unwrap();
        assert!(config.package);
    }

    #[test]
    fn test_parse_target() {
        let config = toml::from_str::<GtpConfig>(
            r#"package = false

[ts]
enabled = true
package = true
tsconfig = { allowImportingTsExtensions = false }
"#,
        )
        .unwrap();

        assert!(!config.package);
        assert_eq!(config.ts.common.package, Some(true));
        assert_eq!(config.py.common.package, None);
    }
}
