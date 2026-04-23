use crate::prelude::internal::*;

mod visitor;
pub use visitor::*;

#[derive(Default)]
// [TODO] Reorganize so that it works like `GtpModuleResolve` with maps organized by module id.
pub struct GtpResolve {
    /// Map of definitions for each module. The definitions can be either root, nested or synthetic
    /// (where the name is derived from the parents). It defines what is exported from the module.
    pub definitions: HashMap<GtModuleId, Vec<GtDefinitionId>>,
    /// Map of imports for each module. It defines what is imported from the other modules.
    pub imports: HashMap<GtModuleId, Vec<GtDefinitionId>>,
    /// Map of local path to module id for each module. It allows to quickly resolve the module id
    /// from any local path.
    pub paths: HashMap<GtModuleId, HashMap<String, GtModuleId>>,
    /// Map of path id to referenced module id.
    pub path_module_ids: HashMap<GtPathModuleId, GtModuleId>,
}

impl GtpResolve {
    pub fn new() -> GtpResolve {
        Default::default()
    }
}

impl TryFrom<&Vec<GtpModuleParse>> for GtpResolve {
    type Error = miette::Error;

    fn try_from(modules_parse: &Vec<GtpModuleParse>) -> Result<GtpResolve> {
        let mut definitions: HashMap<GtModuleId, Vec<GtDefinitionId>> = HashMap::new();
        for module in modules_parse {
            let module_definitions = module
                .1
                .resolve
                .exports
                .iter()
                .map(|export| GtDefinitionId(module.1.module.id.clone(), export.1.clone()))
                .collect();
            definitions.insert(module.1.module.id.clone(), module_definitions);
        }

        let mut paths: HashMap<GtModuleId, HashMap<String, GtModuleId>> = HashMap::new();
        let mut path_module_ids: HashMap<GtPathModuleId, GtModuleId> = HashMap::new();
        let mut imports: HashMap<GtModuleId, Vec<GtDefinitionId>> = HashMap::new();
        for module in modules_parse {
            let mut module_paths: HashMap<String, GtModuleId> = HashMap::new();

            // Manually assign the imports for the package modules
            module.1.module.imports.iter().for_each(|import| {
                if import.path.kind() != GtPathKind::Package {
                    return;
                }

                let package_module_id = GtModuleId(import.path.source_str().to_owned().into());
                let mut definitions = vec![];
                match &import.reference {
                    GtImportReference::Name(_, name) => {
                        definitions.push(GtDefinitionId(package_module_id.clone(), name.1.clone()))
                    }

                    GtImportReference::Names(_, names) => {
                        names.iter().for_each(|name| match name {
                            GtImportName::Name(_, name) => definitions
                                .push(GtDefinitionId(package_module_id.clone(), name.1.clone())),
                            GtImportName::Alias(_, _, alias) => definitions
                                .push(GtDefinitionId(package_module_id.clone(), alias.1.clone())),
                        })
                    }

                    GtImportReference::Glob(_) => {}
                }

                imports
                    .entry(module.1.module.id.clone())
                    .or_default()
                    .extend(definitions.clone());
            });

            for tree_path in module.1.resolve.deps.iter() {
                let module_id: GtModuleId =
                    if let Some(module_id) = module_paths.get(tree_path.source_str()) {
                        // It's already resolved
                        module_id.clone()
                    } else if tree_path.kind() == GtPathKind::Package {
                        // It is a package path
                        let id = GtModuleId(tree_path.source_str().to_owned().into());
                        module_paths.insert(tree_path.source_str().into(), id.clone());
                        id
                    } else {
                        // Get the project module path from the local path
                        let module_path = module.0.resolve(tree_path);
                        let id: GtModuleId = module_path.into();
                        module_paths.insert(tree_path.source_str().into(), id.clone());
                        id
                    };

                path_module_ids.insert(tree_path.id.clone(), module_id.clone());

                if let Some(definitions) = definitions.get(&module_id) {
                    imports
                        .entry(module.1.module.id.clone())
                        .or_default()
                        .extend(definitions.clone());
                }
            }

            paths.insert(module.1.module.id.clone(), module_paths);
        }

        Ok(GtpResolve {
            definitions,
            paths,
            imports,
            path_module_ids,
        })
    }
}
