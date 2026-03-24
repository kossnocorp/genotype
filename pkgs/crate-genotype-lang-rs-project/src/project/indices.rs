use crate::prelude::internal::*;

impl RsProject<'_> {
    pub fn indices_source(&self) -> Vec<GtlProjectFile> {
        let mut crate_paths: IndexMap<GtPkgSrcRelativePath, IndexSet<String>> = IndexMap::new();

        for module in self.modules.iter() {
            let mut module_path = module.path.clone();
            loop {
                let name = module_path.module_name();
                let parent_path = module_path.parent().unwrap_or_else(|| "".into());

                crate_paths
                    .entry(parent_path.clone())
                    .and_modify(|paths| {
                        paths.insert(name.clone());
                    })
                    .or_insert_with(|| IndexSet::from_iter(vec![name]));

                if parent_path == "".into() {
                    break;
                }

                module_path = parent_path;
            }
        }

        crate_paths
            .into_iter()
            .map(|(module_path, modules)| {
                // [TODO] Root path
                // let file_name = "mod.rs";
                let file_name = if module_path == "".into() {
                    "lib.rs"
                } else {
                    "mod.rs"
                };
                let path = self
                    .config
                    .pkg_src_file_path(&module_path.join_path(&file_name.into()));

                let mut code = modules
                    .iter()
                    .map(|module| {
                        format!(
                            r#"pub(crate) mod {module};
pub use {module}::*;"#
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                code += "\n";

                GtlProjectFile { path, source: code }
            })
            .collect()
    }
}

trait Module: GtRelativePath {
    fn module_name(&self) -> String
    where
        Self: Sized,
    {
        self.relative_path()
            .with_extension("")
            .file_name()
            .unwrap_or_default()
            .into()
    }
}

impl Module for GtPkgSrcRelativePath {}
