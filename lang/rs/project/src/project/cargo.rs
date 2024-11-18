use genotype_lang_core_project::source::GTLangProjectSource;
use indexmap::IndexSet;

use super::RSProject;

impl RSProject {
    pub fn cargo_source(&self) -> GTLangProjectSource {
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

        let dependencies = dependencies
            .iter()
            .fold("[dependencies]".into(), |acc, dependency| {
                if let Some(str) = dependency.external_str() {
                    format!("{acc}\n{str}")
                } else {
                    acc
                }
            });

        let cargo = if let Some(cargo) = &self.config.package {
            format!("{cargo}\n")
        } else {
            "".into()
        };

        GTLangProjectSource {
            path: self.config.package_path("Cargo.toml".into()),
            code: format!(
                r#"{cargo}

{dependencies}
"#,
            ),
        }
    }
}
