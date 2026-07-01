use crate::prelude::internal::*;
use heck::ToKebabCase;

mod error;
pub use error::*;

mod format;
pub use format::*;

pub type GtlManifestTypeProjectModule<'project, 'config, Manifest> =
    <Manifest as GtlManifest<'project, 'config>>::ProjectModule;

pub type GtlManifestTypeDependencyIdent<'project, 'config, Manifest> =
    GtlProjectModuleTypeDependencyIdent<
        'config,
        GtlManifestTypeProjectModule<'project, 'config, Manifest>,
    >;

pub type GtlManifestTypeDependencies<'project, 'config, Manifest> =
    IndexSet<GtlManifestTypeDependencyIdent<'project, 'config, Manifest>>;

pub trait GtlManifest<'project, 'config> {
    type ProjectModule: GtlProjectModule;

    fn new(
        config: &'config GtlConfig<'project, GtlProjectModuleTypeLangConfig<Self::ProjectModule>>,
    ) -> Self
    where
        Self: Sized;

    fn config(&self) -> &GtlConfig<'_, GtlProjectModuleTypeLangConfig<Self::ProjectModule>>;

    fn file_name(&self) -> &'static str;

    fn dependencies_key(&self) -> &'static str {
        "dependencies"
    }

    fn name_key(&self) -> &'static str {
        "name"
    }

    fn format_name(&self, name: &str) -> String {
        name.to_kebab_case()
    }

    fn name(&self) -> String {
        let name = self
            .config()
            .lang_config()
            .manifest()
            .get_path(self.name_key())
            .and_then(|value| value.as_str())
            .unwrap_or(self.config().project_name());

        self.format_name(name)
    }

    fn format(&self) -> GtlManifestFormat {
        GtlManifestFormat::Toml
    }

    fn base(&self) -> String {
        "".into()
    }

    fn generate(
        &self,
        deps: &GtlManifestTypeDependencies<'project, 'config, Self>,
    ) -> GtlProjectFileExtra {
        let source_result = Self::generate_source_code(self, deps);
        let target_path = self.config().pkg_file_path(&self.file_name().into());

        match source_result {
            Ok(source) => GtlProjectFileExtraGenerated {
                path: target_path,
                source_code: source,
            }
            .into(),

            Err(err) => GtlProjectFileExtraError::Generate {
                target_path,
                error: err,
            }
            .into(),
        }
    }

    fn generate_source_code(
        &self,
        deps: &GtlManifestTypeDependencies<'project, 'config, Self>,
    ) -> Result<String, Box<dyn GtlError>> {
        let base_manifest = self.base();
        // [TODO]
        let _base_str = base_manifest.clone();

        let mut manifest = DocumentMut::from_str(&base_manifest)
            .map_err(|err| GtlManifestError::toml_parse("the base manifest", err, base_manifest))?;

        let config_manifest_str = self.config().lang_config().manifest().to_string();
        let config_manifest = DocumentMut::from_str(&config_manifest_str).map_err(|err| {
            GtlManifestError::toml_parse("the config manifest", err, config_manifest_str)
        })?;
        // let config_manifest: DocumentMut =
        //     toml_edit::ser::to_document(&self.config().target.manifest())
        //         .map_err(|err| GtlProjectError::ManifestConfigConvert(err))
        //         .into_diagnostic()?;

        // [TODO]
        let _cfg_str = config_manifest.to_string();
        // [TODO]
        let _before_str = manifest.clone().to_string();

        manifest
            .merge(&config_manifest)
            .map_err(|err| GtlManifestError::edit(err))?;

        // [TODO]
        let _after_str = manifest.clone().to_string();

        self.insert_dependencies(&mut manifest, deps)?;

        // [TODO]
        let _final_str = manifest.to_string();

        // let after_insert = manifest.to_string();

        let source = match self.format() {
            GtlManifestFormat::Toml => manifest.to_string(),

            GtlManifestFormat::Json => {
                // [NOTE] To avoid reimplementing the same logic for JSON building, we always use
                // TOML to edit the manifest and then convert it to JSON.
                let toml_val: toml::Value = toml_edit::de::from_document(manifest.clone()).unwrap();
                let json_val = serde_json::to_value(toml_val).unwrap();
                serde_json::to_string_pretty(&json_val)
                    .map_err(|err| GtlManifestError::format_json(err, json_val.to_string()))?
            }
        };
        Ok(source)
    }

    fn insert_dependencies(
        &self,
        manifest: &mut DocumentMut,
        deps: &GtlManifestTypeDependencies<'project, 'config, Self>,
    ) -> Result<(), Box<dyn GtlError>> {
        let manifest_deps = manifest
            .drill_table_mut(self.dependencies_key())
            .map_err(|err| GtlManifestError::edit(err))?;

        for dep in deps.iter() {
            if let Some((key, value)) = Self::dependency_as_kv(dep) {
                manifest_deps[&key] = value.into();
            }
        }

        if manifest_deps.is_empty() {
            manifest.remove(self.dependencies_key());
        }

        Ok(())
    }

    fn dependency_as_kv(
        ident: &GtlProjectModuleTypeDependencyIdent<Self::ProjectModule>,
    ) -> Option<(String, Value)>;
}
