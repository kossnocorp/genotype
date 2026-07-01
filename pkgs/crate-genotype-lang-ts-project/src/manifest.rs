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
        let name = self.name();

        let mut source = format!(
            r#"name = "{name}"
type = "module"
"#
        );

        if let Some(version) = self.config.project_version() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_key() {
        let project_config = GtpConfig::default();
        let project = GtProject::try_new(
            "fallback-name".into(),
            "genotype.toml".into(),
            project_config,
        )
        .unwrap();
        let config = GtlConfig::new(&project, &project.config().ts);
        let manifest = TsManifest::new(&config);

        assert_eq!(manifest.name_key(), "name");
    }

    #[test]
    fn test_name_reads_target_config() {
        let mut project_config = GtpConfig::default();
        project_config.name = Some("Root Name".into());
        project_config.ts.common.manifest = toml::from_str(r#"name = "target-name""#).unwrap();
        let project = GtProject::try_new(
            "fallback-name".into(),
            "genotype.toml".into(),
            project_config,
        )
        .unwrap();
        let config = GtlConfig::new(&project, &project.config().ts);
        let manifest = TsManifest::new(&config);

        assert_eq!(manifest.name(), "target-name");
    }

    #[test]
    fn test_name_falls_back_to_project_name() {
        let mut project_config = GtpConfig::default();
        project_config.name = Some("Root Name".into());
        let project = GtProject::try_new(
            "fallback-name".into(),
            "genotype.toml".into(),
            project_config,
        )
        .unwrap();
        let config = GtlConfig::new(&project, &project.config().ts);
        let manifest = TsManifest::new(&config);

        assert_eq!(manifest.name(), "root-name");
    }
}
