use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct RsProjectModule {
    pub path: GtPkgSrcRelativePath,
    pub module: RsModule,
    pub resolve: RspModuleResolve,
}

impl GtlProjectModule<RsConfig> for RsProjectModule {
    type Dependency = RsDependencyIdent;

    fn generate(config: &RsConfig, module: &GtProjectModule) -> Result<Self> {
        let path = module.path.to_pkg_src_relative_path("rs");

        let mut convert_resolve = RsConvertResolve::default();
        let mut prefixes: HashMap<String, u8> = HashMap::new();

        // [TODO] I'm pretty sure I can extract it and share with TypeScript and Python too
        for import in module.module.imports.iter() {
            if import.path.kind() == GtPathKind::Package {
                convert_resolve.path_module_ids.insert(
                    import.path.id.clone(),
                    GtModuleId(import.path.source_str().to_owned().into()),
                );
            }

            match &import.reference {
                GtImportReference::Glob(_) => {
                    let references = module
                        .resolve
                        .identifiers
                        .iter()
                        .filter(|(_, resolve)| {
                            if let GtpModuleIdentifierSource::External(path) = &resolve.source {
                                return import.path == *path;
                            }
                            false
                        })
                        .collect::<Vec<_>>();

                    if !references.is_empty() {
                        let str = import.path.source_str();
                        let name = str.split('/').next_back().unwrap_or(str).to_string();
                        let prefix = if let Some(count) = prefixes.get(&name) {
                            let prefix = format!("{}{}", name, count);
                            prefixes.insert(name.clone(), count + 1);
                            prefix
                        } else {
                            prefixes.insert(name.clone(), 2);
                            name
                        };

                        convert_resolve
                            .globs
                            .insert(import.path.clone(), prefix.clone());

                        references.iter().for_each(|(reference, _)| {
                            let identifier = (*reference).clone();
                            let span = identifier.0;
                            let alias_str = format!("{}.{}", prefix, identifier.1);
                            let alias = GtIdentifier::new(span, alias_str.into());
                            convert_resolve
                                .identifiers
                                .insert(identifier.clone(), alias);
                            convert_resolve.imported.insert(identifier);
                        });
                    }
                }

                GtImportReference::Name(_, identifier) => {
                    convert_resolve.imported.insert(identifier.clone());
                }

                GtImportReference::Names(_, identifiers) => {
                    identifiers.iter().for_each(|name| {
                        convert_resolve.imported.insert(
                            match name {
                                GtImportName::Name(_, identifier) => identifier,
                                GtImportName::Alias(_, _, identifier) => identifier,
                            }
                            .clone(),
                        );
                    });
                }
            }
        }

        module.resolve.paths.iter().for_each(|(path, module_path)| {
            convert_resolve
                .path_module_ids
                .insert(path.id.clone(), module_path.clone().into());
        });

        convert_resolve.reference_definition_ids = module
            .resolve
            .reference_definition_ids
            .iter()
            .map(|(reference_id, definition_id)| (reference_id.clone(), definition_id.clone()))
            .collect();

        let definitions = module.resolve.definitions.clone();
        let resolve = RspModuleResolve { definitions };

        let module = RsConvertModule::convert(&module.module, &convert_resolve, config)
            .map_err(|err| err.with_source_code(module.source_code.clone()))?
            .0;

        Ok(Self {
            path,
            module,
            resolve,
        })
    }

    fn dependencies(&self) -> Vec<Self::Dependency> {
        self.module
            .imports
            .iter()
            .map(|import| import.dependency.clone())
            .collect()
    }
}

impl Hash for RsProjectModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
