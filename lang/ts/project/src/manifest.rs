use crate::prelude::internal::*;
use toml_edit::*;

impl<'a> GtlProjectManifest<'a> for TsProject<'a> {
    const FILE_NAME: &'static str = "package.json";
    const FORMAT: GtlProjectManifestFormat = GtlProjectManifestFormat::Json;

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

impl GtlProjectManifestDependency for TsProjectManifestDependency {
    type DependencyIdent = TSDependencyIdent;

    fn as_kv(_ident: &Self::DependencyIdent) -> Option<(String, Value)> {
        None
    }
}
