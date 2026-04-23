use crate::prelude::internal::*;
use toml_edit::*;

impl<'a> GtlProjectManifest<'a> for TsProject<'a> {
    const FILE_NAME: &'static str = "package.json";
    const FORMAT: GtlProjectManifestFormat = GtlProjectManifestFormat::Json;

    type Dependency = TsProjectManifestDependency;
    type LangConfig = TsConfig;

    fn config(&'a self) -> &'a GtpConfigPkg<'a, Self::LangConfig> {
        &self.config
    }

    fn base_manifest(&self) -> String {
        let entry = self
            .config
            .pkg_relative_src_file_path(&"index.ts".into())
            .as_str()
            .to_owned();

        let mut source = r#"type = "module"
"#
        .to_string();

        if let Some(version) = self.config.version {
            source.push_str(format!("version = \"{version}\"\n").as_str());
        }

        source.push_str(format!("\n[exports]\n\".\" = \"./{entry}\"\n").as_str());

        source
    }
}

pub struct TsProjectManifestDependency;

impl GtlProjectManifestDependency for TsProjectManifestDependency {
    type DependencyIdent = TsDependencyIdent;

    fn as_kv(ident: &Self::DependencyIdent) -> Option<(String, Value)> {
        match ident {
            TsDependencyIdent::Zod => Some(("zod".into(), "^4".into())),
            TsDependencyIdent::Local(_) => None,
        }
    }
}
