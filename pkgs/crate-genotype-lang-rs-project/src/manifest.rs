use crate::prelude::internal::*;
use genotype_lang_rs_config::RsConfigLang;
use toml_edit::*;

impl<'a> GtlProjectManifest<'a> for RsProject<'a> {
    const FILE_NAME: &'static str = "Cargo.toml";

    type Dependency = RsProjectManifestDependency;
    type LangConfig = RsConfig;

    fn config(&'a self) -> &'a GtpPkgConfig<'a, Self::LangConfig> {
        &self.config
    }

    fn base_manifest(&self) -> String {
        let mut source = format!(
            r#"[package]
edition = "{}"
"#,
            RsConfigLang::DEFAULT_EDITION
        );

        if let Some(version) = self.config.version {
            source.push_str(format!("version = \"{version}\"\n").as_str());
        }

        source.push('\n');
        source
    }
}

pub struct RsProjectManifestDependency;

impl RsProjectManifestDependency {
    fn dependency_value(version: &'static str, features: Vec<&'static str>) -> Value {
        if features.is_empty() {
            version.into()
        } else {
            let mut table = InlineTable::new();
            table.insert("version", version.into());
            let features = Value::Array(Array::from_iter(features));
            table.insert("features", features);
            Value::InlineTable(table)
        }
    }
}

impl GtlProjectManifestDependency for RsProjectManifestDependency {
    type DependencyIdent = RsDependencyIdent;

    fn as_kv(ident: &Self::DependencyIdent) -> Option<(String, Value)> {
        match ident {
            Self::DependencyIdent::Runtime => Some(("genotype_runtime".into(), "0.4".into())),
            Self::DependencyIdent::Litty => Some(("litty".into(), "0.3".into())),
            Self::DependencyIdent::Serde => {
                Some(("serde".into(), Self::dependency_value("1", vec!["derive"])))
            }
            Self::DependencyIdent::OrderedFloat => Some((
                "ordered-float".into(),
                Self::dependency_value("5", vec!["serde"]),
            )),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn test_ordered_float_dependency() {
        let (_, value) =
            RsProjectManifestDependency::as_kv(&RsDependencyIdent::OrderedFloat).unwrap();

        assert_snapshot!(
            value.to_string(),
            @r#"{ version = "5", features = ["serde"] }"#
        );
    }
}
