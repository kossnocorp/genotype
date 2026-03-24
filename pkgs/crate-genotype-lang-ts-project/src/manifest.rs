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

    fn base_manifest(&self) -> String {
        let types = self
            .config
            .pkg_relative_src_file_path(&"index.ts".into())
            .as_str()
            .to_owned();
        format!(
            r#"types = "{types}"
"#
        )
    }
}

pub struct TsProjectManifestDependency;

impl GtlProjectManifestDependency for TsProjectManifestDependency {
    type DependencyIdent = TSDependencyIdent;

    fn as_kv(_ident: &Self::DependencyIdent) -> Option<(String, Value)> {
        None
    }
}
