use crate::prelude::internal::*;
use toml_edit::*;

impl<'a> GtlProjectManifest<'a> for TsProject<'a> {
    const FILE_NAME: &'static str = "Cargo.toml";

    type Dependency = TsProjectManifestDependency;
    type LangConfig = TsConfig;

    fn config(&'a self) -> &'a GtConfigPkg<'a, Self::LangConfig> {
        &self.config
    }

    fn alter_manifest_doc(&self, doc: &mut DocumentMut) {
        doc.insert(
            "types",
            self.config
                .pkg_relative_src_file_path(&"index.ts".into())
                .as_str()
                .into(),
        );
    }
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
