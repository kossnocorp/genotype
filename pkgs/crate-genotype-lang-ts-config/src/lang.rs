use crate::prelude::internal::*;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TsConfigLang {
    pub tsconfig: TsConfigLangTsconfig,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TsConfigLangTsconfig {
    #[serde(rename = "allowImportingTsExtensions")]
    pub allow_importing_ts_extensions: bool,
}

impl TsConfigLang {
    pub fn format_module_path(&self, path: &GtPkgSrcRelativePath) -> String {
        let mut path = path.as_str().to_string();
        if !self.tsconfig.allow_importing_ts_extensions {
            if path.ends_with(".ts") {
                let len = path.len();
                path.replace_range(len - 2..len, "js");
            }
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
    fn format_module_path() {
        let config = TsConfigLang {
            tsconfig: TsConfigLangTsconfig {
                allow_importing_ts_extensions: true,
            },
        };
        assert_eq!(
            config.format_module_path(&"path/to/module.ts".into()),
            "path/to/module.ts"
        );

        let config = TsConfigLang {
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
            tsconfig: TsConfigLangTsconfig {
                allow_importing_ts_extensions: true,
            },
        };
        assert_eq!(config.format_import_path("foo"), "foo.ts");

        let config = TsConfigLang {
            tsconfig: TsConfigLangTsconfig {
                allow_importing_ts_extensions: false,
            },
        };
        assert_eq!(config.format_import_path("foo"), "foo.js");
    }
}
