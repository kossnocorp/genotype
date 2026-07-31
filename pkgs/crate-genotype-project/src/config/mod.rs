use crate::prelude::internal::*;

mod error;
pub use error::*;

mod toml_str;

mod manifest;
pub use manifest::*;

pub const GTCONFIG_FILE: &str = "genotype.toml";

const fn default_package() -> bool {
    true
}

const fn default_warning_comment() -> bool {
    true
}

const fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct GtpBuildConfig {
    /// Whether to resolve and update `genotype.build.toml`.
    #[serde(default = "default_true")]
    pub file: bool,
    /// Whether to remove unchanged generated files no longer produced by a build.
    #[serde(default = "default_true")]
    pub cleanup: bool,
}

impl Default for GtpBuildConfig {
    fn default() -> Self {
        Self {
            file: true,
            cleanup: true,
        }
    }
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
    /// Generated file tracking and cleanup options.
    #[serde(default)]
    pub build: GtpBuildConfig,
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
    /// Global formatters to run after all selected targets are compiled.
    #[serde(default)]
    pub formatters: Vec<GtpFormatter>,
    #[serde(default = "default_warning_comment")]
    pub warning_comment: bool,
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
    source_code: GtpSourceCode,
}

impl Default for GtpConfig {
    fn default() -> Self {
        Self {
            name: None,
            version: None,
            package: true,
            build: Default::default(),
            root: Default::default(),
            dist: Default::default(),
            src: Default::default(),
            entry: Default::default(),
            formatters: Default::default(),
            ts: Default::default(),
            py: Default::default(),
            rs: Default::default(),
            source_code: Default::default(),
            warning_comment: true,
        }
    }
}

impl GtpConfig {
    pub fn parse(source: String) -> Result<Self> {
        Self::from_source_code(GtpSourceCode::new(source))
    }

    pub fn source(&self) -> &GtpSourceCode {
        &self.source_code
    }

    pub fn health_check(&self) -> Vec<GtDiagnostic> {
        if !self.build.file && self.build.cleanup {
            vec![GtDiagnostic::warning(
                "`build.cleanup` has no effect when `build.file` is disabled",
            )]
        } else {
            vec![]
        }
    }

    pub fn lang(&self, lang: GtpLang) -> &dyn GtpLangConfig {
        match lang {
            GtpLang::Py => &self.py,
            GtpLang::Rs => &self.rs,
            GtpLang::Ts => &self.ts,
        }
    }

    pub fn lang_package_enabled(&self, lang_config: &dyn GtpLangConfig) -> bool {
        lang_config.common().package.unwrap_or(self.package)
    }

    pub fn lang_enabled(&self, lang: GtpLang) -> bool {
        match lang {
            GtpLang::Py => self.python_enabled(),
            GtpLang::Rs => self.rust_enabled(),
            GtpLang::Ts => self.ts_enabled(),
        }
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
            source_code: Default::default(),
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
            source_code: Default::default(),
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
    fn test_build_defaults() {
        let config = toml::from_str::<GtpConfig>("").unwrap();
        assert!(config.build.file);
        assert!(config.build.cleanup);
    }

    #[test]
    fn test_parse_build_options() {
        let config = toml::from_str::<GtpConfig>(
            r#"[build]
file = true
cleanup = false
"#,
        )
        .unwrap();
        assert!(config.build.file);
        assert!(!config.build.cleanup);
    }

    #[test]
    fn test_build_cleanup_without_file_warning() {
        let config = GtpConfig::parse(
            r#"[build]
file = false
cleanup = true
"#
            .into(),
        )
        .unwrap();

        assert_eq!(
            config.health_check(),
            vec![GtDiagnostic::warning(
                "`build.cleanup` has no effect when `build.file` is disabled"
            )]
        );
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
ext = "ts"
"#,
        )
        .unwrap();

        assert!(!config.package);
        assert_eq!(config.ts.common.package, Some(true));
        assert_eq!(config.ts.lang.ext, TsImportExt::Ts);
        assert_eq!(config.py.common.package, None);
    }

    #[test]
    fn test_parse_ts_ext() {
        let config = GtpConfig::from_toml_str(
            r#"[ts]
ext = "none"
tsconfig = { include = ["src/**/*.ts"] }
"#,
        )
        .unwrap();

        assert_eq!(config.ts.lang.ext, TsImportExt::None);
        let tsconfig = config.ts.lang.tsconfig.as_ref().unwrap();
        assert_eq!(
            tsconfig.get("include"),
            Some(&toml::Value::Array(vec![toml::Value::String(
                "src/**/*.ts".into()
            )]))
        );
    }

    #[test]
    fn test_parse_root_formatters() {
        let config = toml::from_str::<GtpConfig>(
            r#"formatters = [
  { kind = "shell", cmd = "npm", args = ["run", "format"] },
  { kind = "pnpm", cmd = "prettier", args = ["--check", "."] },
  { kind = "cargo", cmd = "fmt", args = ["--all"] },
  { kind = "prettyplease" },
  { kind = "oxfmt", via = "pnpm" },
  { kind = "prettier", via = "npx" },
]
"#,
        )
        .unwrap();

        assert_ron_snapshot!(config.formatters, @r#"
        [
          __GtpFormatterShellLiteralsSerialize(
            cmd: "npm",
            args: Some([
              "run",
              "format",
            ]),
            kind: "shell",
          ),
          GtpFormatterExecutor(
            cmd: "prettier",
            args: Some([
              "--check",
              ".",
            ]),
            kind: "pnpm",
          ),
          GtpFormatterExecutor(
            cmd: "fmt",
            args: Some([
              "--all",
            ]),
            kind: "cargo",
          ),
          __GtpFormatterPresetPrettypleaseLiteralsSerialize(
            kind: "prettyplease",
          ),
          __GtpFormatterPresetOxfmtLiteralsSerialize(
            via: Some("pnpm"),
            kind: "oxfmt",
          ),
          __GtpFormatterPresetPrettierLiteralsSerialize(
            via: Some("npx"),
            kind: "prettier",
          ),
        ]
        "#);
    }

    #[test]
    fn test_parse_target_formatters() {
        let config = toml::from_str::<GtpConfig>(
            r#"[py]
enabled = true
version = "latest"
formatters = [{ kind = "ruff", via = "uv" }]
"#,
        )
        .unwrap();

        assert_ron_snapshot!(config.py.common.formatters, @r#"
        [
          __GtpFormatterPresetRuffLiteralsSerialize(
            via: Some("uv"),
            kind: "ruff",
          ),
        ]
        "#);
    }
}
