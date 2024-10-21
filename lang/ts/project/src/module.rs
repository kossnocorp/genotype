use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    path::PathBuf,
};

use genotype_lang_core_project::module::GTLangProjectModule;
use genotype_lang_ts_config::TSProjectConfig;
use genotype_lang_ts_converter::{module::TSConvertModule, resolve::TSConvertResolve};
use genotype_lang_ts_tree::module::TSModule;
use genotype_parser::{tree::GTImportReference, GTIdentifier};
use genotype_project::{module::GTProjectModule, GTProject, GTProjectModuleReference};

#[derive(Debug, PartialEq, Clone)]
pub struct TSProjectModule {
    pub path: PathBuf,
    pub module: TSModule,
}

impl GTLangProjectModule<TSProjectConfig> for TSProjectModule {
    fn generate(
        project: &GTProject,
        module: &GTProjectModule,
        config: &TSProjectConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let path = config.source_path(
            module
                .path
                .as_path()
                .strip_prefix(project.root.as_path())?
                .with_extension("ts"),
        );

        let mut resolve = TSConvertResolve::new();
        let mut prefixes: HashMap<String, u8> = HashMap::new();

        for import in module.module.imports.iter() {
            if let GTImportReference::Glob(_) = import.reference {
                let references = module
                    .resolve
                    .references
                    .iter()
                    .filter(|(_, reference)| {
                        if let GTProjectModuleReference::External(path) = reference {
                            return import.path == *path;
                        }
                        false
                    })
                    .collect::<Vec<_>>();

                if references.len() > 0 {
                    let str = import.path.as_str();
                    let name = str.split('/').last().unwrap_or(str).to_string();
                    let prefix = if let Some(count) = prefixes.get(&name) {
                        let prefix = format!("{}{}", name, count);
                        prefixes.insert(name.clone(), count + 1);
                        prefix
                    } else {
                        prefixes.insert(name.clone(), 2);
                        name
                    };

                    resolve.globs.insert(import.path.clone(), prefix.clone());

                    references.iter().for_each(|(reference, _)| {
                        let identifier = (*reference).clone();
                        let span = identifier.0.clone();
                        let alias = format!("{}.{}", prefix, identifier.1);
                        resolve
                            .identifiers
                            .insert(identifier, GTIdentifier::new(span, alias));
                    });
                }
            }
        }

        let module = TSConvertModule::convert(&module.module, &resolve).0;

        Ok(Self { path, module })
    }
}

impl Hash for TSProjectModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
