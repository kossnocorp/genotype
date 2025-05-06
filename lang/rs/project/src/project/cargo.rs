use crate::prelude::internal::*;

impl RSProject {
    pub fn cargo_source(&self) -> Result<GTLangProjectSource> {
        let cargo_src = if let Some(cargo) = &self.config.package {
            cargo
        } else {
            ""
        };
        let mut cargo: toml_edit::DocumentMut = cargo_src
            .parse()
            .map_err(|err| RSProjectError::ParsePackage(err))
            .into_diagnostic()?;

        let cargo_dependencies = cargo["dependencies"].as_table_mut();

        let cargo_dependencies = if let Some(deps) = cargo_dependencies {
            deps
        } else {
            cargo.insert(
                "dependencies",
                toml_edit::Item::Table(toml_edit::Table::new()),
            );
            cargo["dependencies"].as_table_mut().ok_or_else(|| {
                RSProjectError::EditCargo("Failed to create dependencies table".into())
            })?
        };

        let dependencies = self
            .modules
            .iter()
            .flat_map(|module| {
                module
                    .module
                    .imports
                    .iter()
                    .map(|import| import.dependency.clone())
            })
            .collect::<IndexSet<_>>();

        for ident in dependencies.iter() {
            if let Some(external) = ident.external() {
                let version = toml_edit::Value::String(toml_edit::Formatted::new(external.version));

                cargo_dependencies.insert(
                    &external.name,
                    if external.features.is_empty() {
                        toml_edit::Item::Value(version)
                    } else {
                        let mut table = toml_edit::InlineTable::new();
                        table.insert("version", version);
                        let features =
                            toml_edit::Value::Array(toml_edit::Array::from_iter(external.features));
                        table.insert("features", features);
                        toml_edit::Item::Value(toml_edit::Value::InlineTable(table))
                    },
                );
            }
        }

        Ok(GTLangProjectSource {
            path: self.config.package_path("Cargo.toml".into()),
            code: cargo.to_string(),
        })
    }
}
