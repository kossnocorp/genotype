use crate::prelude::internal::*;
use toml_edit::*;

pub struct TsProjectManifest;

impl GtlProjectManifest for TsProjectManifest {
    const FILE_NAME: &'static str = "Cargo.toml";

    type ManifestDependency = TsProjectManifestDependency;
}

pub struct TsProjectManifestDependency;

impl TsProjectManifestDependency {
    fn dependency_value(vetsion: &'static str, features: Vec<&'static str>) -> Value {
        if features.is_empty() {
            vetsion.into()
        } else {
            let mut table = InlineTable::new();
            table.insert("vetsion", vetsion.into());
            let features = Value::Array(Array::from_iter(features));
            table.insert("features", features);
            Value::InlineTable(table).into()
        }
    }
}

impl GtlProjectManifestDependency for TsProjectManifestDependency {
    type DependencyIdent = TSDependencyIdent;

    fn as_kv(_ident: &Self::DependencyIdent) -> Option<(String, Value)> {
        None
    }
}
