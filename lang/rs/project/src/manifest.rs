use crate::prelude::internal::*;
use toml_edit::*;

impl<'a> GtlProjectManifest<'a> for RsProject<'a> {
    const FILE_NAME: &'static str = "Cargo.toml";

    type Dependency = RsProjectManifestDependency;
    type LangConfig = RsConfig;

    fn config(&'a self) -> &'a GtConfigPkg<'a, Self::LangConfig> {
        &self.config
    }

    fn base_manifest(&self) -> String {
        format!(
            r#"[package]
edition = "2024"

"#
        )
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
            Value::InlineTable(table).into()
        }
    }
}

impl GtlProjectManifestDependency for RsProjectManifestDependency {
    type DependencyIdent = RSDependencyIdent;

    fn as_kv(ident: &Self::DependencyIdent) -> Option<(String, Value)> {
        match ident {
            Self::DependencyIdent::Runtime => Some(("genotype_runtime".into(), "0.4".into())),
            Self::DependencyIdent::Litty => Some(("litty".into(), "0.2".into())),
            Self::DependencyIdent::Serde => {
                Some(("serde".into(), Self::dependency_value("1", vec!["derive"])))
            }
            _ => None,
        }
    }
}
