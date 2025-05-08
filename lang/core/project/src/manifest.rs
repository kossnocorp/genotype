use crate::prelude::internal::*;
use miette::IntoDiagnostic;
use serde_json::to_string_pretty;
use toml_edit::*;

pub trait GtlProjectManifest<'a> {
    const FILE_NAME: &'static str;
    const DEPENDENCIES_KEY: &'static str = "dependencies";
    const FORMAT: GtlProjectManifestFormat = GtlProjectManifestFormat::Toml;

    type Dependency: GtlProjectManifestDependency;
    type LangConfig: GtlConfig;

    fn config(&'a self) -> &'a GtConfigPkg<'a, Self::LangConfig>;

    fn alter_manifest_doc(&self, _doc: &mut DocumentMut) {}

    fn base_manifest(&self) -> DocumentMut {
        DocumentMut::new()
    }

    fn generate_manifest(
        &'a self,
        deps: &'a Vec<
            <<Self as GtlProjectManifest<'a>>::Dependency as GtlProjectManifestDependency>::DependencyIdent,
        >,
    ) -> Result<GtlProjectFile> {
        let mut manifest = self.base_manifest();

        // let manifest_deps = if let Some(deps) = manifest[Self::DEPENDENCIES_KEY].as_table_mut() {
        //     deps
        // } else {
        //     manifest.insert(Self::DEPENDENCIES_KEY, Item::Table(Table::new()));
        //     manifest[Self::DEPENDENCIES_KEY]
        //         .as_table_mut()
        //         .ok_or_else(|| {
        //             GtlProjectError::ManifestAdjust(
        //                 Self::FILE_NAME,
        //                 format!(r#"can't create "{}" table"#, Self::DEPENDENCIES_KEY),
        //             )
        //         })?
        // };

        {
            let config_manifest: DocumentMut =
                toml_edit::ser::to_document(&self.config().target.manifest()).into_diagnostic()?;

            for (key, item) in config_manifest.iter() {
                if key == Self::DEPENDENCIES_KEY {
                    let deps = manifest
                        .entry(Self::DEPENDENCIES_KEY)
                        .or_insert(Item::Table(Table::new()))
                        .as_table_mut()
                        .ok_or_else(|| GtlProjectError::ManifestDepsAccess(Self::FILE_NAME))?;
                    let config_deps = item
                        .as_table()
                        .ok_or_else(|| GtlProjectError::ManifestDepsAccess(Self::FILE_NAME))?;
                    deps.extend(config_deps.clone());
                } else {
                    manifest.insert(key, item.clone());
                }
            }
        }

        let manifest_deps = manifest
            .entry(Self::DEPENDENCIES_KEY)
            .or_insert(Item::Table(Table::new()))
            .as_table_mut()
            .ok_or_else(|| GtlProjectError::ManifestDepsAccess(Self::FILE_NAME))?;

        // let manifest_deps =
        //     if let Some(deps) = config_manifest[Self::DEPENDENCIES_KEY].as_table_mut() {
        //         deps
        //     } else {
        //         config_manifest.insert(Self::DEPENDENCIES_KEY, Item::Table(Table::new()));
        //         config_manifest[Self::DEPENDENCIES_KEY]
        //             .as_table_mut()
        //             .ok_or_else(|| {
        //                 GtlProjectError::ManifestDepsAccess(
        //                     Self::FILE_NAME,
        //                     format!(r#"can't create "{}" table"#, Self::DEPENDENCIES_KEY),
        //                 )
        //             })?
        //     };

        for dep in deps.iter() {
            if let Some((key, value)) = Self::Dependency::as_kv(dep) {
                manifest_deps.insert(&key, value.into());
            }
        }

        if manifest_deps.is_empty() {
            manifest.remove(Self::DEPENDENCIES_KEY);
        }

        self.alter_manifest_doc(&mut manifest);

        let source = match Self::FORMAT {
            GtlProjectManifestFormat::Toml => manifest.to_string(),

            GtlProjectManifestFormat::Json => {
                // [NOTE] To avoid reimplementing the same logic for JSON building, we always use
                // TOML to edit the manifest and then convert it to JSON.
                let toml_val: toml::Value = toml_edit::de::from_document(manifest.clone()).unwrap();
                let json_val = serde_json::to_value(toml_val).unwrap();
                to_string_pretty(&json_val)
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
