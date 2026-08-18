use crate::prelude::internal::*;

impl GtpConfig {
    pub fn from_toml_str(source_code: &str) -> Result<Self> {
        Self::from_source_code(GtpSourceCode::new(source_code.to_owned()))
    }

    pub fn from_source_code(source_code: GtpSourceCode) -> Result<Self> {
        let mut config: GtpConfig = Figment::from(figment::providers::Serialized::defaults(
            GtpConfig::default(),
        ))
        .merge(figment::providers::Toml::string(&source_code.content))
        .extract()
        .into_diagnostic()?;
        config.source_code = source_code;

        Ok(config)
    }

    pub fn to_toml_str(&self) -> Result<String> {
        toml::to_string(self)
            .map_err(|_| GtpConfigError::FailedToStringify)
            .into_diagnostic()
    }

    pub fn to_toml_str_pruned(&self) -> Result<String> {
        let current_str = self.to_toml_str()?;
        let mut current_doc = DocumentMut::from_str(&current_str).into_diagnostic()?;

        let defaults_str = GtpConfig::default().to_toml_str()?;
        let defaults_doc = DocumentMut::from_str(&defaults_str)
            .map_err(|_| GtpConfigError::FailedToStringify)
            .into_diagnostic()?;

        let original_doc = DocumentMut::from_str(&self.source_code.content).ok();

        current_doc.as_table_mut().prune_defaults(
            defaults_doc.as_table(),
            original_doc.as_ref().map(|doc| doc.as_table()),
        );
        let pruned_str = current_doc.to_string();

        Ok(pruned_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_ts_naming_source_dir_inheritance_round_trip() {
        let config = GtpConfig::from_toml_str(
            r#"[ts.naming]
source_file = "kebab-case"
"#,
        )
        .unwrap();

        assert_eq!(config.ts.lang.naming.source_dir, None);
        assert_eq!(
            config.ts.lang.naming.source_dir(),
            GtpLangConfigNamingCase::KebabCase
        );
        assert_snapshot!(
            config.to_toml_str_pruned().unwrap(),
            @r#"

        [ts]

        [ts.naming]
        source_file = "kebab-case"
        "#
        );
    }
}
