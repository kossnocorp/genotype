use std::path::PathBuf;

use genotype_lang_core_project::source::GTLangProjectSource;
use indexmap::{IndexMap, IndexSet};

use super::RSProject;

impl RSProject {
    pub fn indices_source(&self) -> Vec<GTLangProjectSource> {
        let src_root = self.config.src_path();
        let mut crate_paths: IndexMap<PathBuf, IndexSet<String>> = IndexMap::new();

        for module in self.modules.iter() {
            let mut module: PathBuf = module.path.clone();
            loop {
                let path: PathBuf = module.parent().unwrap().into();
                let name = module
                    .with_extension("")
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned();

                crate_paths
                    .entry(path.clone())
                    .and_modify(|paths| {
                        paths.insert(name.clone());
                    })
                    .or_insert_with(|| IndexSet::from_iter(vec![name]));

                if path == src_root {
                    break;
                }

                module = path;
            }
        }

        crate_paths
            .into_iter()
            .map(|(module_path, modules)| {
                let path = module_path.join(if src_root == module_path {
                    "lib.rs"
                } else {
                    "mod.rs"
                });

                let code = modules
                    .iter()
                    .map(|module| format!("pub mod {};", module))
                    .collect::<Vec<_>>()
                    .join("\n");

                GTLangProjectSource { path, code }
            })
            .collect()
    }
}
