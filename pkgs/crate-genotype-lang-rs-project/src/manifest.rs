use crate::prelude::internal::*;
use genotype_lang_rs_config::RsConfigLang;
use toml_edit::*;

pub struct RsManifest<'project, 'config> {
    config: &'config GtlConfig<'project, RsConfig>,
}

impl<'project, 'config> GtlManifest<'project, 'config> for RsManifest<'project, 'config> {
    type ProjectModule = RsProjectModule;

    fn new(
        config: &'config GtlConfig<'project, GtlProjectModuleTypeLangConfig<Self::ProjectModule>>,
    ) -> Self
    where
        Self: Sized,
    {
        RsManifest { config }
    }

    fn config(&self) -> &GtlConfig<'_, GtlProjectModuleTypeLangConfig<Self::ProjectModule>> {
        self.config
    }

    fn file_name(&self) -> &'static str {
        "Cargo.toml"
    }

    fn base(&self) -> String {
        let mut source = format!(
            r#"[package]
edition = "{}"
"#,
            RsConfigLang::DEFAULT_EDITION
        );

        if let Some(version) = self.config.project_version {
            source.push_str(format!("version = \"{version}\"\n").as_str());
        }

        source.push('\n');
        source
    }

    fn dependency_as_kv(
        ident: &GtlProjectModuleTypeDependencyIdent<Self::ProjectModule>,
    ) -> Option<(String, Value)> {
        match ident {
            RsDependencyIdent::Runtime => Some(("genotype_runtime".into(), "0.4".into())),
            RsDependencyIdent::Litty => Some(("litty".into(), "0.5".into())),
            RsDependencyIdent::Serde => {
                Some(("serde".into(), dependency_value("1", vec!["derive"])))
            }
            RsDependencyIdent::OrderedFloat => {
                Some(("ordered-float".into(), dependency_value("5", vec!["serde"])))
            }
            _ => None,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn test_ordered_float_dependency() {
        let (_, value) = RsManifest::dependency_as_kv(&RsDependencyIdent::OrderedFloat).unwrap();

        assert_snapshot!(
            value.to_string(),
            @r#"{ version = "5", features = ["serde"] }"#
        );
    }
}
