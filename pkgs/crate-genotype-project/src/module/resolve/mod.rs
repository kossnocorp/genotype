use crate::prelude::internal::*;

// region: Modules

mod identifier;
pub use identifier::*;

mod definition;
pub use definition::*;

mod visitor;
pub use visitor::*;

// endregion

// region: Module resolving

impl GtpModule {
    /// Resolves the project module.
    pub fn resolve(self, project_resolve: &GtpResolve) -> GtpModule {
        let project_module_parse = match self {
            GtpModule::Parsed(project_module_parse) => project_module_parse,
            GtpModule::Resolved(state) => state.project_module_parse,
            GtpModule::Error(_) => return self,
            GtpModule::Initialized => return GtpModule::Error(GtpModuleError::ResolveInitialized),
        };

        GtpModuleResolve::resolve(project_resolve, &project_module_parse).map_or_else(
            GtpModule::Error,
            |resolve| {
                GtpModuleResolved {
                    project_module_parse,
                    resolve,
                }
                .into()
            },
        )
    }
}

// endregion

/// region: Resolved module state

/// Resolved project module state.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpModuleResolved {
    /// Project module parse.
    pub project_module_parse: GtpModuleParse,

    /// Project module resolve.
    pub resolve: GtpModuleResolve,
}

impl Into<GtpModule> for GtpModuleResolved {
    fn into(self) -> GtpModule {
        GtpModule::Resolved(self)
    }
}

// endregion

// region: Module resolve

/// Module resolve data. It describes relations between module entities. It allows to
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpModuleResolve {
    /// Paths resolve.
    pub paths: IndexMap<GtPath, GtpSrcDirRelativeModulePath>,
    /// Identifiers resolve.
    pub identifiers: IndexMap<GtIdentifier, GtpModuleResolveIdentifier>,
    /// Definitions resolve.
    pub definitions: IndexMap<GtDefinitionId, GtpModuleResolveDefinition>,
    /// Reference id to definition id resolve.
    pub reference_definition_ids: IndexMap<GtReferenceId, GtDefinitionId>,
}

impl GtpModuleResolve {
    pub fn resolve(
        project_resolve: &GtpResolve,
        module_parse: &GtpModuleParse,
    ) -> Result<Self, GtpModuleError> {
        let module_id = module_parse.module_parse.module.id.clone();

        // Resolve module dependencies by mapping local paths to project module paths.
        let mut paths: IndexMap<GtPath, GtpSrcDirRelativeModulePath> = IndexMap::new();
        for local_path in module_parse.module_parse.resolve.deps.iter() {
            // Continue if the dependency is already resolved or it is a package path.
            if paths.contains_key(local_path) || local_path.kind() == GtPathKind::Package {
                continue;
            }

            let Some(dep_module_id) = project_resolve
                .modules
                .get(&module_id)
                .and_then(|module| module.paths.get(local_path.source_str()))
            else {
                continue;
            };

            let dep_path = GtpSrcDirRelativeModulePath::new(
                RelativePathBuf::from(dep_module_id.0.as_ref()).with_extension("type"),
            );
            paths.insert(local_path.clone(), dep_path);
        }

        // Resolve module references mapping identifiers to dependencies.
        let mut identifiers: IndexMap<GtIdentifier, GtpModuleResolveIdentifier> = IndexMap::new();
        for reference in module_parse.module_parse.resolve.references.iter() {
            // Continue if the reference is already resolved.
            if identifiers.contains_key(reference) {
                continue;
            };

            // Check if the reference is a package import.
            let package_import = module_parse
                .module_parse
                .module
                .imports
                .iter()
                .find(|import| {
                    if import.path.kind() != GtPathKind::Package {
                        return false;
                    }
                    match &import.reference {
                        GtImportReference::Name(_, name) => name.1 == reference.1,

                        GtImportReference::Names(_, names) => names.iter().any(|name| match name {
                            GtImportName::Name(_, name) => name.1 == reference.1,
                            GtImportName::Alias(_, _, alias) => alias.1 == reference.1,
                        }),

                        GtImportReference::Glob(_) => false,
                    }
                });
            if let Some(import) = package_import {
                identifiers.insert(
                    reference.clone(),
                    GtpModuleResolveIdentifier {
                        source: GtpModuleResolveIdentifierSource::Package(import.path.clone()),
                    },
                );
                continue;
            }

            // Check if the reference is local.
            if module_parse
                .module_parse
                .resolve
                .exports
                .iter()
                .any(|export| export.1 == reference.1)
            {
                identifiers.insert(
                    reference.clone(),
                    GtpModuleResolveIdentifier {
                        source: GtpModuleResolveIdentifierSource::Local,
                    },
                );
                continue;
            }

            // Find the local path of the reference.
            let (local_path, _) = paths
                .iter()
                .find(|(local_path, _module_path)| {
                    let import = module_parse
                        .module_parse
                        .module
                        .imports
                        .iter()
                        .find(|import| {
                            if import.path != **local_path {
                                return false;
                            }

                            match &import.reference {
                                GtImportReference::Glob(_) => {
                                    let Some(dep_module_id) =
                                        project_resolve.modules.get(&module_id).and_then(
                                            |module| module.paths.get(local_path.source_str()),
                                        )
                                    else {
                                        return false;
                                    };

                                    let Some(dep_module) =
                                        project_resolve.modules.get(dep_module_id)
                                    else {
                                        return false;
                                    };

                                    dep_module
                                        .definitions
                                        .iter()
                                        .any(|definition| definition.1 == reference.1)
                                }

                                GtImportReference::Name(_, name) => name.1 == reference.1,

                                GtImportReference::Names(_, names) => {
                                    names.iter().any(|name| match name {
                                        GtImportName::Name(_, name) => name.1 == reference.1,

                                        GtImportName::Alias(_, _, alias) => alias.1 == reference.1,
                                    })
                                }
                            }
                        });
                    import.is_some()
                })
                .ok_or_else(|| GtpModuleError::Resolve {
                    path: module_parse.path.clone(),
                    error: GtpError::UndefinedType {
                        span: reference.as_span(),
                        identifier: reference.as_string(),
                    },
                })?;

            identifiers.insert(
                reference.clone(),
                GtpModuleResolveIdentifier {
                    source: GtpModuleResolveIdentifierSource::External(local_path.clone()),
                },
            );
        }

        let mut resolve = GtpModuleResolve {
            paths,
            identifiers,
            definitions: Default::default(),
            reference_definition_ids: Default::default(),
        };

        let mut visitor = GtpModuleResolveVisitor::new(module_id, project_resolve);
        module_parse.module_parse.module.traverse(&mut visitor);

        if let Some(error) = visitor.error() {
            return Err(GtpModuleError::Resolve {
                path: module_parse.path.clone(),
                error: error.clone(),
            });
        }

        resolve.definitions = visitor.drain_definitions();
        resolve.reference_definition_ids = visitor.get_reference_definition_ids();

        Ok(resolve)
    }

}

// endregion
