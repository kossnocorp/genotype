use crate::prelude::internal::*;

pub mod visitor;

#[derive(Default)]
// [TODO] Reorganize so that it works like `GTPModuleResolve` with maps organized by module id.
pub struct GTPResolve {
    /// Map of definitions for each module. The definitions can be either root, nested or synthetic
    /// (where the name is derived from the parents). It defines what is exported from the module.
    pub definitions: HashMap<GTModuleId, Vec<GTDefinitionId>>,
    /// Map of imports for each module. It defines what is imported from the other modules.
    pub imports: HashMap<GTModuleId, Vec<GTDefinitionId>>,
    /// Map of local path to module id for each module. It allows to quickly resolve the module id
    /// from any local path.
    pub paths: HashMap<GTModuleId, HashMap<String, GTModuleId>>,
}

impl GTPResolve {
    pub fn new() -> GTPResolve {
        Default::default()
    }
}

impl TryFrom<&Vec<GTProjectModuleParse>> for GTPResolve {
    type Error = miette::Error;

    fn try_from(modules_parse: &Vec<GTProjectModuleParse>) -> Result<GTPResolve> {
        let mut definitions: HashMap<GTModuleId, Vec<GTDefinitionId>> = HashMap::new();
        for module in modules_parse {
            let module_definitions = module
                .1
                .resolve
                .exports
                .iter()
                .map(|export| GTDefinitionId(module.1.module.id.clone(), export.1.clone()))
                .collect();
            definitions.insert(module.1.module.id.clone(), module_definitions);
        }

        let mut paths: HashMap<GTModuleId, HashMap<String, GTModuleId>> = HashMap::new();
        let mut imports: HashMap<GTModuleId, Vec<GTDefinitionId>> = HashMap::new();
        for module in modules_parse {
            let mut module_paths: HashMap<String, GTModuleId> = HashMap::new();

            // Manually assign the imports for the package modules
            module.1.module.imports.iter().for_each(|import| {
                if import.path.kind() != GTPathKind::Package {
                    return;
                }

                let package_module_id = GTModuleId(import.path.source_str().to_owned());
                let mut definitions = vec![];
                match &import.reference {
                    GTImportReference::Name(_, name) => {
                        definitions.push(GTDefinitionId(package_module_id.clone(), name.1.clone()))
                    }

                    GTImportReference::Names(_, names) => {
                        names.iter().for_each(|name| match name {
                            GTImportName::Name(_, name) => definitions
                                .push(GTDefinitionId(package_module_id.clone(), name.1.clone())),
                            GTImportName::Alias(_, _, alias) => definitions
                                .push(GTDefinitionId(package_module_id.clone(), alias.1.clone())),
                        })
                    }

                    GTImportReference::Glob(_) => {}
                }

                imports
                    .entry(module.1.module.id.clone())
                    .or_insert(Default::default())
                    .extend(definitions.clone());
            });

            for tree_path in module.1.resolve.deps.iter() {
                let module_id: GTModuleId =
                    if let Some(module_id) = module_paths.get(tree_path.source_str()) {
                        // It's already resolved
                        module_id.clone()
                    } else if tree_path.kind() == GTPathKind::Package {
                        // It is a package path
                        let id = GTModuleId(tree_path.source_str().to_owned());
                        module_paths.insert(tree_path.source_str().into(), id.clone());
                        id
                    } else {
                        // Get the project module path from the local path
                        let module_path = module.0.resolve(tree_path);
                        let id: GTModuleId = module_path.into();
                        module_paths.insert(tree_path.source_str().into(), id.clone());
                        id
                    };

                if let Some(definitions) = definitions.get(&module_id) {
                    imports
                        .entry(module.1.module.id.clone())
                        .or_insert(Default::default())
                        .extend(definitions.clone());
                }
            }

            paths.insert(module.1.module.id.clone(), module_paths);
        }

        Ok(GTPResolve {
            definitions,
            paths,
            imports,
        })
    }
}
