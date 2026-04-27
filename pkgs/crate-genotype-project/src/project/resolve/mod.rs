use crate::prelude::internal::*;

// region: Modules

mod module;
pub use module::*;

// endregion

// region: Project resolving

impl GtProject {
    /// Resolves the project modules.
    pub fn resolve_modules(&mut self) -> Result<()> {
        let project_resolve = GtpResolve::resolve(&self.modules)?;

        for module in self.modules.values_mut() {
            let current_module = std::mem::replace(module, GtpModule::Initialized);
            *module = current_module.resolve(&project_resolve);
        }

        Ok(())
    }

    /// Sorts the project modules by their paths. This is useful for deterministic output and
    /// testing.
    pub fn sort_modules(&mut self) {
        self.modules.sort_keys();
    }
}

// endregion

// region: Project resolve data

#[derive(Default)]
pub struct GtpResolve {
    /// Resolve data mapped by module id.
    pub modules: IndexMap<GtModuleId, GtpResolveModule>,
    /// Map of path id to referenced module id.
    pub path_module_ids: IndexMap<GtPathModuleId, GtModuleId>,
}

impl GtpResolve {
    pub fn new() -> GtpResolve {
        Default::default()
    }

    pub fn resolve(modules: &IndexMap<GtpModulePath, GtpModule>) -> Result<GtpResolve> {
        let mut resolve_modules: IndexMap<GtModuleId, GtpResolveModule> = IndexMap::new();
        let mut module_ids_by_path: IndexMap<GtpModulePath, GtModuleId> = IndexMap::new();

        for (module_path, module) in modules {
            let Some(parse) = module.module_parse() else {
                continue;
            };

            module_ids_by_path.insert(module_path.clone(), parse.module.id.clone());

            let module_definitions = parse
                .resolve
                .exports
                .iter()
                .map(|export| GtDefinitionId(parse.module.id.clone(), export.1.clone()))
                .collect();

            resolve_modules
                .entry(parse.module.id.clone())
                .or_default()
                .definitions = module_definitions;
        }

        let mut path_module_ids: IndexMap<GtPathModuleId, GtModuleId> = IndexMap::new();

        for (module_path, module) in modules {
            let Some(parse) = module.module_parse() else {
                continue;
            };

            let mut module_paths: IndexMap<String, GtModuleId> = IndexMap::new();
            let mut module_imports: Vec<GtDefinitionId> = Vec::new();

            // Manually assign the imports for the package modules
            parse.module.imports.iter().for_each(|import| {
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

                module_imports.extend(definitions.clone());
            });

            for tree_path in parse.resolve.deps.iter() {
                let module_id: GtModuleId = if let Some(module_id) =
                    module_paths.get(tree_path.source_str())
                {
                    // It's already resolved
                    module_id.clone()
                } else if tree_path.kind() == GtPathKind::Package {
                    // It is a package path
                    let id = GtModuleId(tree_path.source_str().to_owned().into());
                    module_paths.insert(tree_path.source_str().to_owned(), id.clone());
                    id
                } else {
                    // Get the project module path from the local path
                    let dep_module_path = module_path.resolve_path_node(tree_path);
                    let id = module_ids_by_path
                        .get(&dep_module_path)
                        .cloned()
                        .unwrap_or_else(|| GtModuleId(tree_path.source_str().to_owned().into()));
                    module_paths.insert(tree_path.source_str().to_owned(), id.clone());
                    id
                };

                path_module_ids.insert(tree_path.id.clone(), module_id.clone());

                if let Some(module_resolve) = resolve_modules.get(&module_id) {
                    module_imports.extend(module_resolve.definitions.clone());
                }
            }

            let module_resolve = resolve_modules.entry(parse.module.id.clone()).or_default();
            module_resolve.imports = module_imports;
            module_resolve.paths = module_paths;
        }

        Ok(GtpResolve {
            modules: resolve_modules,
            path_module_ids,
        })
    }
}

// endregion
