use crate::prelude::internal::*;

impl RsConfig {
    pub const RUST_EDITION: &'static str = "2024";

    pub fn rust_edition_health_check(
        &self,
        config_path: &GtpConfigFilePath,
        package_enabled: bool,
    ) -> Option<GtDiagnostic> {
        if self.has_locked_edition() || !package_enabled {
            return None;
        }

        let title = indoc::formatdoc! {
            r#"Rust edition is not locked in {config_path}.

              Quick fix:

              [rs.manifest.package]
              edition = "{edition}"
            "#,
            edition = RsConfigLang::DEFAULT_EDITION
        };

        Some(GtDiagnostic {
            kind: GtDiagnosticKind::Warning,
            content: title.into(),
        })
    }

    fn has_locked_edition(&self) -> bool {
        self.edition().is_some()
    }

    pub fn edition(&self) -> Option<&str> {
        self.common
            .manifest
            .get_path("package.edition")
            .and_then(|edition| edition.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_edition_some() {
        let config = toml::from_str::<RsConfig>(
            r#"enabled = true

[manifest.package]
edition = "2024"
"#,
        )
        .unwrap();

        assert_ron_snapshot!(
            config.edition(),
            @r#"Some("2024")"#
        );

        assert!(config.has_locked_edition());

        assert_ron_snapshot!(
            config.health_check(&"./genotype.toml".into(), true),
            @"[]"
        );
    }

    #[test]
    fn test_edition_none() {
        let config = toml::from_str::<RsConfig>(
            r#"enabled = true

[manifest.package]
"#,
        )
        .unwrap();

        assert_ron_snapshot!(
            config.edition(),
            @"None"
        );
        assert!(!config.has_locked_edition());

        assert_ron_snapshot!(
            config.health_check(&"./genotype.toml".into(), true),
            @r#"
        [
          GtDiagnostic(
            kind: "warning",
            content: GtDiagnosticContentMessage(
              title: "Rust edition is not locked in genotype.toml.\n\nQuick fix:\n\n[rs.manifest.package]\nedition = \"2024\"\n",
            ),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_edition_none_custom_config_path() {
        let config = toml::from_str::<RsConfig>(
            r#"enabled = true

[manifest.package]
"#,
        )
        .unwrap();

        assert_ron_snapshot!(
            config.edition(),
            @"None"
        );
        assert!(!config.has_locked_edition());

        assert_ron_snapshot!(
            config.health_check(&"./genotype.lib.toml".into(), true),
            @r#"
        [
          GtDiagnostic(
            kind: "warning",
            content: GtDiagnosticContentMessage(
              title: "Rust edition is not locked in genotype.lib.toml.\n\nQuick fix:\n\n[rs.manifest.package]\nedition = \"2024\"\n",
            ),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_edition_none_package_disabled() {
        let config = toml::from_str::<RsConfig>(
            r#"enabled = true

[manifest.package]
"#,
        )
        .unwrap();

        assert_ron_snapshot!(
            config.edition(),
            @"None"
        );
        assert!(!config.has_locked_edition());

        assert_ron_snapshot!(
            config.health_check(&"./genotype.lib.toml".into(), false),
            @"[]"
        );
    }
}
