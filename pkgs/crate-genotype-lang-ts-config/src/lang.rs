use crate::prelude::internal::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct TsConfigLang {
    #[serde(default)]
    pub mode: TsMode,
    #[serde(default)]
    pub prefer: TsPrefer,
    #[serde(default)]
    pub ext: TsImportExt,
    #[serde(default)]
    pub tsconfig: Option<TsConfigLangTsconfig>,
    #[serde(default)]
    pub naming: TsConfigNaming,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct TsConfigNaming {
    #[serde(default)]
    pub source_file: GtpLangConfigNamingCase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_dir: Option<GtpLangConfigNamingCase>,
}

impl TsConfigNaming {
    pub fn source_dir(&self) -> GtpLangConfigNamingCase {
        self.source_dir.unwrap_or(self.source_file)
    }

    pub fn format_module_path(&self, path: &str) -> String {
        GtpLangConfigNamingCase::format_file_path(path, self.source_dir(), self.source_file)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum TsMode {
    #[default]
    Types,
    Zod,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum TsPrefer {
    #[default]
    Interface,
    Alias,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum TsImportExt {
    #[default]
    Js,
    Ts,
    None,
}

pub type TsConfigLangTsconfig = toml::Table;

impl TsConfigLang {
    pub fn format_module_path(&self, path: &GtpPkgSrcDirRelativePath) -> String {
        let path = path.as_str();
        let Some(path) = path.strip_suffix(".ts") else {
            return path.to_string();
        };
        match self.ext {
            TsImportExt::Js => format!("{path}.js"),
            TsImportExt::Ts => format!("{path}.ts"),
            TsImportExt::None => path.to_string(),
        }
    }

    pub fn format_import_path(&self, path: &str) -> String {
        match self.ext {
            TsImportExt::Js => format!("{path}.js"),
            TsImportExt::Ts => format!("{path}.ts"),
            TsImportExt::None => path.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_default_prefer_interface() {
        assert_eq!(TsConfigLang::default().prefer, TsPrefer::Interface);
    }

    #[test]
    fn test_default_tsconfig() {
        assert_eq!(TsConfigLang::default().tsconfig, None);
    }

    #[test]
    fn test_naming_defaults() {
        let naming = TsConfigNaming::default();
        assert_eq!(naming.source_file, GtpLangConfigNamingCase::CamelCase);
        assert_eq!(naming.source_dir(), GtpLangConfigNamingCase::CamelCase);
        assert_eq!(naming.source_dir, None);
    }

    #[test]
    fn test_naming_source_dir_defaults_to_source_file() {
        let config: TsConfig = toml::from_str(
            r#"
                [naming]
                source_file = "kebab-case"
            "#,
        )
        .unwrap();

        assert_eq!(
            config.lang.naming.source_file,
            GtpLangConfigNamingCase::KebabCase
        );
        assert_eq!(
            config.lang.naming.source_dir(),
            GtpLangConfigNamingCase::KebabCase
        );
        assert_eq!(config.lang.naming.source_dir, None);
    }

    #[test]
    fn test_naming_source_dir_override() {
        let config: TsConfig = toml::from_str(
            r#"
                [naming]
                source_file = "PascalCase"
                source_dir = "snake_case"
            "#,
        )
        .unwrap();

        assert_eq!(
            config.lang.naming.source_file,
            GtpLangConfigNamingCase::PascalCase
        );
        assert_eq!(
            config.lang.naming.source_dir(),
            GtpLangConfigNamingCase::SnakeCase
        );
    }

    #[test]
    fn test_naming_rejects_invalid_case() {
        assert!(
            toml::from_str::<TsConfig>(
                r#"
                    [naming]
                    source_file = "camel_case"
                "#,
            )
            .is_err()
        );
    }

    #[test]
    fn test_format_module_path() {
        let config = TsConfigLang {
            ext: TsImportExt::Ts,
            ..Default::default()
        };
        assert_eq!(
            config.format_module_path(&"path/to/module.ts".into()),
            "path/to/module.ts"
        );

        let config = TsConfigLang {
            ext: TsImportExt::Js,
            ..Default::default()
        };
        assert_eq!(
            config.format_module_path(&"path/to/module.ts".into()),
            "path/to/module.js"
        );

        let config = TsConfigLang {
            ext: TsImportExt::None,
            ..Default::default()
        };
        assert_eq!(
            config.format_module_path(&"path/to/module.ts".into()),
            "path/to/module"
        );
    }

    #[test]
    fn test_format_import_path() {
        let config = TsConfigLang {
            ext: TsImportExt::Ts,
            ..Default::default()
        };
        assert_eq!(config.format_import_path("foo"), "foo.ts");

        let config = TsConfigLang {
            ext: TsImportExt::Js,
            ..Default::default()
        };
        assert_eq!(config.format_import_path("foo"), "foo.js");

        let config = TsConfigLang {
            ext: TsImportExt::None,
            ..Default::default()
        };
        assert_eq!(config.format_import_path("foo"), "foo");
    }
}
