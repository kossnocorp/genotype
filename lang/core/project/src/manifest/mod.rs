use crate::prelude::internal::*;
use miette::IntoDiagnostic;
use std::str::FromStr;
use toml_edit::*;

mod toml_ext;
use toml_ext::*;

pub trait GtlProjectManifest<'a> {
    const FILE_NAME: &'static str;
    const MANIFEST_DEPENDENCIES_KEY: &'static str = "dependencies";
    const FORMAT: GtlProjectManifestFormat = GtlProjectManifestFormat::Toml;

    type Dependency: GtlProjectManifestDependency;
    type LangConfig: GtlConfig;

    fn config(&'a self) -> &'a GtConfigPkg<'a, Self::LangConfig>;

    fn base_manifest(&self) -> String {
        "".into()
    }

    fn generate_manifest(
        &'a self,
        deps: &'a Vec<
            <<Self as GtlProjectManifest<'a>>::Dependency as GtlProjectManifestDependency>::DependencyIdent,
        >,
    ) -> Result<GtlProjectFile> {
        let base_manifest = self.base_manifest();
        // [TODO]
        let base_str = base_manifest.clone();

        let mut manifest = DocumentMut::from_str(&base_manifest)
            .map_err(|err| GtlProjectError::ManifestTomlParse("the base manifest", err))
            .into_diagnostic()
            .map_err(|err| err.with_source_code(base_manifest))?;

        let config_manifest_str = self.config().target.manifest().to_string();
        let config_manifest = DocumentMut::from_str(&config_manifest_str)
            .map_err(|err| GtlProjectError::ManifestTomlParse("the config manifest", err))
            .into_diagnostic()
            .map_err(|err| err.with_source_code(config_manifest_str))?;
        // let config_manifest: DocumentMut =
        //     toml_edit::ser::to_document(&self.config().target.manifest())
        //         .map_err(|err| GtlProjectError::ManifestConfigConvert(err))
        //         .into_diagnostic()?;

        // [TODO]
        let cfg_str = config_manifest.to_string();
        // [TODO]
        let before_str = manifest.clone().to_string();

        manifest.merge(&config_manifest)?;

        // [TODO]
        let after_str = manifest.clone().to_string();

        let manifest_deps = manifest.drill_table_mut(Self::MANIFEST_DEPENDENCIES_KEY)?;

        // let manifest_deps = manifest
        //     .entry(Self::MANIFEST_DEPENDENCIES_KEY)
        //     .or_insert(Item::Table(Table::new()))
        //     .as_table_mut()
        //     .ok_or_else(|| GtlProjectError::ManifestDepsAccess(Self::FILE_NAME))?;

        for dep in deps.iter() {
            if let Some((key, value)) = Self::Dependency::as_kv(dep) {
                manifest_deps[&key] = value.into();
            }
        }

        if manifest_deps.is_empty() {
            manifest.remove(Self::MANIFEST_DEPENDENCIES_KEY);
        }

        // [TODO]
        let final_str = manifest.to_string();

        // let after_insert = manifest.to_string();

        let source = match Self::FORMAT {
            GtlProjectManifestFormat::Toml => manifest.to_string(),

            GtlProjectManifestFormat::Json => {
                // [NOTE] To avoid reimplementing the same logic for JSON building, we always use
                // TOML to edit the manifest and then convert it to JSON.
                let toml_val: toml::Value = toml_edit::de::from_document(manifest.clone()).unwrap();
                let json_val = serde_json::to_value(toml_val).unwrap();
                serde_json::to_string_pretty(&json_val)
                    .map_err(|_| GtlProjectError::ManifestFormat(Self::FILE_NAME))
                    .into_diagnostic()
                    .map_err(|err| err.with_source_code(json_val.to_string()))?
            }
        };

        Ok(GtlProjectFile {
            path: self.config().pkg_file_path(&Self::FILE_NAME.into()),
            source,
        })
    }
}

pub enum GtlProjectManifestFormat {
    Toml,
    Json,
}

pub trait GtlProjectManifestDependency {
    type DependencyIdent: GtlDependencyIdent;

    fn as_kv(ident: &Self::DependencyIdent) -> Option<(String, Value)>;
}
