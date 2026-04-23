use crate::prelude::internal::*;

/// Module resolve data. It describes relations between module entities. It allows to
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpModuleResolve {
    /// Paths resolve.
    pub paths: IndexMap<GtPath, GtpModulePath>,
    /// Identifiers resolve.
    pub identifiers: IndexMap<GtIdentifier, GtpModuleIdentifierResolve>,
    /// Definitions resolve.
    pub definitions: IndexMap<GtDefinitionId, GtpModuleDefinitionResolve>,
    /// Reference id to definition id resolve.
    pub reference_definition_ids: IndexMap<GtReferenceId, GtDefinitionId>,
}

impl GtpModuleResolve {
    pub fn try_new(modules: &[GtpModuleParse], parse: &GtpModuleParse) -> Result<Self> {
        // Resolve module dependencies by mapping local paths to project module paths
        let mut paths: IndexMap<GtPath, GtpModulePath> = IndexMap::new();
        for local_path in parse.1.resolve.deps.iter() {
            // Continue if the dependency is already resolved or it is a package path
            if paths.contains_key(local_path) || local_path.kind() == GtPathKind::Package {
                continue;
            }

            // Get the project module path from the local path
            let module_path = parse.0.resolve(local_path);
            paths.insert(local_path.clone(), module_path);
        }

        // Resolve module references mapping identifiers to dependencies
        let mut identifiers: IndexMap<GtIdentifier, GtpModuleIdentifierResolve> = IndexMap::new();
        for reference in parse.1.resolve.references.iter() {
            // Continue if the reference is already resolved
            if identifiers.contains_key(reference) {
                continue;
            };

            // Check if the reference is a package import
            let package_import = parse.1.module.imports.iter().find(|import| {
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
                    GtpModuleIdentifierResolve {
                        source: GtpModuleIdentifierSource::Package(import.path.clone()),
                    },
                );
                continue;
            }

            // Check if the reference is local
            if parse
                .1
                .resolve
                .exports
                .iter()
                .any(|export| export.1 == reference.1)
            {
                identifiers.insert(
                    reference.clone(),
                    GtpModuleIdentifierResolve {
                        source: GtpModuleIdentifierSource::Local,
                    },
                );
                continue;
            }

            // Find the local path of the reference
            let (local_path, _) = paths
                .iter()
                .find(|(local_path, module_resolve)| {
                    let import = parse.1.module.imports.iter().find(|import| {
                        if import.path != **local_path {
                            return false;
                        }

                        match &import.reference {
                            GtImportReference::Glob(_) => {
                                let module = modules
                                    .iter()
                                    .find(|module| module.0 == **module_resolve)
                                    .unwrap();
                                module
                                    .1
                                    .resolve
                                    .exports
                                    .iter()
                                    .any(|export| export.1 == reference.1)
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
                .ok_or_else(|| GtpError::UndefinedType {
                    span: reference.as_span(),
                    identifier: reference.as_string(),
                })?;

            identifiers.insert(
                reference.clone(),
                GtpModuleIdentifierResolve {
                    source: GtpModuleIdentifierSource::External(local_path.clone()),
                },
            );
        }

        Ok(GtpModuleResolve {
            paths,
            identifiers,
            definitions: Default::default(),
            reference_definition_ids: Default::default(),
        })
    }
}
