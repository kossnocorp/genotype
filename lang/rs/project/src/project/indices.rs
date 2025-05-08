use std::iter::Successors;

use crate::prelude::internal::*;

impl RsProject<'_> {
    pub fn indices_source(&self) -> Vec<GtlProjectFile> {
        let mut crate_paths: IndexMap<GtPkgSrcRelativePath, IndexSet<String>> = IndexMap::new();

        for module in self.modules.iter() {
            let mut module_path = module.path.clone();
            loop {
                let cur_path = module_path.parent();
                let name = module_path.module_name();

                let key = if let Some(ref path) = cur_path {
                    path.clone()
                } else {
                    "".into()
                };

                crate_paths
                    .entry(key)
                    .and_modify(|paths| {
                        paths.insert(name.clone());
                    })
                    .or_insert_with(|| IndexSet::from_iter(vec![name]));

                match cur_path {
                    Some(ref path) => {
                        module_path = path.clone();
                    }
                    None => break,
                }
            }
        }

        println!("crate_paths: {crate_paths:?}");

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
        self.relative_path()
            .with_extension("")
            .file_name()
            .unwrap_or_default()
            .into()
    }

    // fn modules_hierarchy(&self) -> Successors<Self, fn(&Self) -> Option<Self>>
    // where
    //     Self: Sized + Clone,
    // {
    //     std::iter::successors(Some(self.clone()), Self::parent)
    // }
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
