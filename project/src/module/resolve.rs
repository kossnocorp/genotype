use std::{collections::HashMap, sync::Arc};

use genotype_parser::tree::{GTIdentifier, GTImportName, GTImportReference, GTPath};

use super::{GTProjectModuleParse, GTProjectModulePath};

#[derive(Debug, PartialEq, Clone)]
pub struct GTProjectModuleResolve {
    pub deps: HashMap<GTPath, Arc<GTProjectModulePath>>,
    pub references: HashMap<GTIdentifier, GTProjectModuleReference>,
}

impl GTProjectModuleResolve {
    pub fn try_new(
        modules: &Vec<GTProjectModuleParse>,
        parse: &GTProjectModuleParse,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Resolve module dependencies by mapping local paths to project module paths
        let mut deps = HashMap::new();
        for local_path in parse.1.resolve.deps.iter() {
            // Continue if the dependency is already resolved
            if deps.contains_key(local_path) {
                continue;
            }

            // Get the project module path from the local path
            let path = Arc::new(parse.0.resolve(local_path)?);
            deps.insert(local_path.clone(), path);
        }

        // Resolve module references mapping identifiers to dependencies
        let mut references = HashMap::new();
        for reference in parse.1.resolve.references.iter() {
            // Continue if the reference is already resolved
            if references.contains_key(reference) {
                continue;
            };

            // Check if the reference is local
            if parse
                .1
                .resolve
                .exports
                .iter()
                .any(|export| export.1 == reference.1)
            {
                references.insert(reference.clone(), GTProjectModuleReference::Local);
                continue;
            }

            // Find the local path of the reference
            let (local_path, _) = deps
                .iter()
                .find(|(local_path, path)| {
                    let import = parse.1.module.imports.iter().find(|import| {
                        if import.path != **local_path {
                            return false;
                        }

                        match &import.reference {
                            GTImportReference::Glob(_) => {
                                let module =
                                    modules.iter().find(|module| module.0 == ***path).unwrap();
                                module
                                    .1
                                    .resolve
                                    .exports
                                    .iter()
                                    .any(|export| export.1 == reference.1)
                            }

                            GTImportReference::Name(_, name) => name.1 == reference.1,

                            GTImportReference::Names(_, names) => {
                                names.iter().any(|name| match name {
                                    GTImportName::Name(_, name) => name.1 == reference.1,

                                    GTImportName::Alias(_, _, alias) => alias.1 == reference.1,
                                })
                            }
                        }
                    });
                    !import.is_none()
                })
                .unwrap();

            references.insert(
                reference.clone(),
                GTProjectModuleReference::External(local_path.clone()),
            );
        }

        Ok(GTProjectModuleResolve { deps, references })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GTProjectModuleReference {
    Local,
    External(GTPath),
}
