use crate::prelude::internal::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct TsConfigLang {
    #[serde(default)]
    pub mode: TsMode,
    #[serde(default)]
    pub prefer: TsPrefer,
    pub tsconfig: TsConfigLangTsconfig,
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

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TsConfigLangTsconfig {
    #[serde(rename = "allowImportingTsExtensions")]
    pub allow_importing_ts_extensions: bool,
}

impl TsConfigLang {
    pub fn format_module_path(&self, path: &GtPkgSrcRelativePath) -> String {
        let mut path = path.as_str().to_string();
        if !self.tsconfig.allow_importing_ts_extensions && path.ends_with(".ts") {
            let len = path.len();
            path.replace_range(len - 2..len, "js");
        }
        path
    }

    pub fn format_import_path(&self, path: &str) -> String {
        let ext = if self.tsconfig.allow_importing_ts_extensions {
            "ts"
        } else {
            "js"
        };
        format!("{path}.{ext}")
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
    fn format_module_path() {
        let config = TsConfigLang {
            mode: Default::default(),
            prefer: Default::default(),
            tsconfig: TsConfigLangTsconfig {
                allow_importing_ts_extensions: true,
            },
        };
        assert_eq!(
            config.format_module_path(&"path/to/module.ts".into()),
            "path/to/module.ts"
        );

        let config = TsConfigLang {
            mode: Default::default(),
            prefer: Default::default(),
            tsconfig: TsConfigLangTsconfig {
                allow_importing_ts_extensions: false,
            },
        };
        assert_eq!(
            config.format_module_path(&"path/to/module.ts".into()),
            "path/to/module.js"
        );
    }

    #[test]
    fn format_import_path() {
        let config = TsConfigLang {
            mode: Default::default(),
            prefer: Default::default(),
            tsconfig: TsConfigLangTsconfig {
                allow_importing_ts_extensions: true,
            },
        };
        assert_eq!(config.format_import_path("foo"), "foo.ts");

        let config = TsConfigLang {
            mode: Default::default(),
            prefer: Default::default(),
            tsconfig: TsConfigLangTsconfig {
                allow_importing_ts_extensions: false,
            },
        };
        assert_eq!(config.format_import_path("foo"), "foo.js");
    }
}
