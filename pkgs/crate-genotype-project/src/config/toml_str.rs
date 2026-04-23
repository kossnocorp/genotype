use crate::prelude::internal::*;

impl GtpConfig {
    pub fn from_toml_str(source: &str) -> Result<Self> {
        let mut config: GtpConfig = Figment::from(figment::providers::Serialized::defaults(
            GtpConfig::default(),
        ))
        .merge(figment::providers::Toml::string(source))
        .extract()
        .into_diagnostic()?;
        config.source_toml_str = source.to_string();

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

        let original_doc = DocumentMut::from_str(&self.source_toml_str).ok();

        current_doc.as_table_mut().prune_defaults(
            defaults_doc.as_table(),
            original_doc.as_ref().map(|doc| doc.as_table()),
        );
        let pruned_str = current_doc.to_string();

        Ok(pruned_str)
    }
}
