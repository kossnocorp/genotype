use crate::prelude::internal::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct TsConfigLang {
    #[serde(default)]
    pub mode: TsMode,
    #[serde(default)]
    pub prefer: TsPrefer,
    #[serde(default, rename = "import_ext")]
    pub ext: TsImportExt,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Default)]
#[serde(rename_all = "snake_case")]
pub enum TsImportExt {
    #[default]
    Js,
    Ts,
    None,
}

impl TsConfigLang {
    pub fn format_module_path(&self, path: &GtpPkgSrcDirRelativePath) -> String {
        let path = path.as_str();
        let stem = path.strip_suffix(".ts").unwrap_or(path);
        match self.ext {
            TsImportExt::Js => format!("{stem}.js"),
            TsImportExt::Ts => format!("{stem}.ts"),
            TsImportExt::None => stem.to_string(),
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
    fn test_default_import_ext_js() {
        assert_eq!(TsConfigLang::default().ext, TsImportExt::Js);
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
