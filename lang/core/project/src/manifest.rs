use crate::prelude::internal::*;
use miette::IntoDiagnostic;
use toml_edit::*;

pub trait GtlProjectManifest {
    const FILE_NAME: &'static str;
    const DEPENDENCIES_KEY: &'static str = "dependencies";
    const MANIFEST_FORMAT: GtlProjectManifestFormat = GtlProjectManifestFormat::Toml;

    type ManifestDependency: GtlProjectManifestDependency;

    fn manifest_file<Lang: GtlConfig>(
        config: &GtConfigPkg<'_, Lang>,
        deps: &Vec<
            <<Self as GtlProjectManifest>::ManifestDependency as GtlProjectManifestDependency>::DependencyIdent,
        >,
    ) -> Result<GtlProjectFile> {
        Self::manifest_file_with_edits(config, deps, |_| {})
    }

    fn manifest_file_with_edits<Lang: GtlConfig>(
        config: &GtConfigPkg<'_, Lang>,
        deps: &Vec<
            <<Self as GtlProjectManifest>::ManifestDependency as GtlProjectManifestDependency>::DependencyIdent,
        >,
        edit: impl FnOnce(&mut DocumentMut),
    ) -> Result<GtlProjectFile> {
        let mut manifest: DocumentMut =
            toml_edit::ser::to_document(&config.target.manifest()).into_diagnostic()?;

        let manifest_deps = if let Some(deps) = manifest[Self::DEPENDENCIES_KEY].as_table_mut() {
            deps
        } else {
            manifest.insert(Self::DEPENDENCIES_KEY, Item::Table(Table::new()));
            manifest[Self::DEPENDENCIES_KEY]
                .as_table_mut()
                .ok_or_else(|| {
                    GtlProjectError::ManifestAdjust(
                        Self::FILE_NAME,
                        format!(r#"can't create "{}" table"#, Self::DEPENDENCIES_KEY),
                    )
                })?
        };

        for dep in deps.iter() {
            if let Some((key, value)) = Self::ManifestDependency::as_kv(dep) {
                manifest_deps.insert(&key, value.into());
            }
        }

        edit(&mut manifest);

        let source = match Self::MANIFEST_FORMAT {
            GtlProjectManifestFormat::Toml => manifest.to_string(),

            GtlProjectManifestFormat::Json => {
                // [NOTE] To avoid reimplementing the same logic for JSON building, we always use
                // TOML to edit the manifest and then convert it to JSON.
                let toml_val: toml::Value = toml_edit::de::from_document(manifest.clone()).unwrap();
                let json_val = serde_json::to_value(toml_val).unwrap();
                json_val.to_string()
            }
        };

        Ok(GtlProjectFile {
            path: config.pkg_file_path(&Self::FILE_NAME.into()),
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
