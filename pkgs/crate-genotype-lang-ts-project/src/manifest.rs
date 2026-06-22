use crate::prelude::internal::*;
use toml_edit::*;

pub struct TsManifest<'project, 'config> {
    config: &'config GtlConfig<'project, TsConfig>,
}

impl<'project, 'config> GtlManifest<'project, 'config> for TsManifest<'project, 'config> {
    type ProjectModule = TsProjectModule;

    fn new(
        config: &'config GtlConfig<'project, GtlProjectModuleTypeLangConfig<Self::ProjectModule>>,
    ) -> Self
    where
        Self: Sized,
    {
        TsManifest { config }
    }

    fn config(&self) -> &GtlConfig<'_, GtlProjectModuleTypeLangConfig<Self::ProjectModule>> {
        self.config
    }

    fn file_name(&self) -> &'static str {
        "package.json"
    }

    fn format(&self) -> GtlManifestFormat {
        GtlManifestFormat::Json
    }

    fn base(&self) -> String {
        let entry = self
            .config
            .pkg_relative_src_file_path(&"index.ts".into())
            .as_str()
            .to_owned();

        let mut source = r#"type = "module"
"#
        .to_string();

        if let Some(version) = self.config.project_version {
            source.push_str(format!("version = \"{version}\"\n").as_str());
        }

        source.push_str(format!("\n[exports]\n\".\" = \"./{entry}\"\n").as_str());

        source
    }

    fn dependency_as_kv(ident: &TsDependencyIdent) -> Option<(String, Value)> {
        match ident {
            TsDependencyIdent::Zod => Some(("zod".into(), "^4".into())),
            TsDependencyIdent::Local(_) => None,
        }
    }
}
