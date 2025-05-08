use std::iter::Successors;

use crate::prelude::internal::*;

impl RsProject<'_> {
    pub fn indices_source(&self) -> Vec<GtlProjectFile> {
        let mut crate_paths: IndexMap<GtPkgSrcRelativePath, IndexSet<String>> = IndexMap::new();

        for module in self.modules.iter() {
            for module_path in module.path.modules_hierarchy() {
                let name = module_path.module_name();
                crate_paths
                    .entry(module_path)
                    .and_modify(|paths| {
                        paths.insert(name.clone());
                    })
                    .or_insert_with(|| IndexSet::from_iter(vec![name]));
            }
        }

        crate_paths
            .into_iter()
            .map(|(module_path, modules)| {
                // [TODO] Root path
                let file_name = "mod.rs";
                // let file_name = if src_root == module_path {
                //     "lib.rs"
                // } else {
                //     "mod.rs"
                // };
                let path = self
                    .config
                    .pkg_src_file_path(&module_path.join_path(&file_name.into()));

                let mut code = modules
                    .iter()
                    .map(|module| {
                        format!(
                            r#"mod {module};
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
        self.strip_extension().as_str().into()
    }

    fn modules_hierarchy(&self) -> Successors<Self, fn(&Self) -> Option<Self>>
    where
        Self: Sized + Clone,
    {
        std::iter::successors(Some(self.clone()), Self::parent)
    }
}

impl Module for GtPkgSrcRelativePath {
    // fn module_name(&self) -> String {
    //     self.strip_extension().as_str().into()
    // }

    // fn module_names_hierarchy(&self) -> Successors<Self, fn(&Self) -> Option<Self>>
    // where
    //     Self: Sized,
    // {
    //     std::iter::successors(self.parent(), Self::parent).map(|path| path.strip_extension())
    // }
}
