use std::collections::HashMap;

use genotype_parser::{
    tree::{GTIdentifier, GTImportName, GTImportReference, GTPath},
    GTDefinitionId,
};
use miette::Result;

use crate::error::GTProjectError;

use super::{
    identifier::GTPModuleIdentifierSource, GTPModuleDefinitionResolve, GTPModuleIdentifierResolve,
    GTPModulePathResolve, GTProjectModuleParse,
};

/// Module resolve data. It describes relations between module entities. It allows to
#[derive(Debug, PartialEq, Clone)]
pub struct GTPModuleResolve {
    /// Paths resolve.
    pub paths: HashMap<GTPath, GTPModulePathResolve>,
    /// Identifiers resolve.
    pub identifiers: HashMap<GTIdentifier, GTPModuleIdentifierResolve>,
    /// Definitions resolve.
    pub definitions: HashMap<GTDefinitionId, GTPModuleDefinitionResolve>,
}

impl GTPModuleResolve {
    pub fn try_new(
        modules: &Vec<GTProjectModuleParse>,
        parse: &GTProjectModuleParse,
    ) -> Result<Self> {
        // Resolve module dependencies by mapping local paths to project module paths
        let mut paths: HashMap<GTPath, GTPModulePathResolve> = HashMap::new();
        for local_path in parse.1.resolve.deps.iter() {
            // Continue if the dependency is already resolved
            if paths.contains_key(local_path) {
                continue;
            }

            // Get the project module path from the local path
            let module_path = parse
                .0
                .resolve(local_path)
                .map_err(|_| GTProjectError::CannotResolve(local_path.as_str().to_owned()))?;
            paths.insert(local_path.clone(), GTPModulePathResolve { module_path });
        }

        // Resolve module references mapping identifiers to dependencies
        let mut identifiers: HashMap<GTIdentifier, GTPModuleIdentifierResolve> = HashMap::new();
        for reference in parse.1.resolve.references.iter() {
            // Continue if the reference is already resolved
            if identifiers.contains_key(reference) {
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
                identifiers.insert(
                    reference.clone(),
                    GTPModuleIdentifierResolve {
                        source: GTPModuleIdentifierSource::Local,
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
                            GTImportReference::Glob(_) => {
                                let module = modules
                                    .iter()
                                    .find(|module| module.0 == module_resolve.module_path)
                                    .unwrap();
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
                .ok_or_else(|| GTProjectError::UndefinedType {
                    span: reference.as_span(),
                    identifier: reference.as_string(),
                })?;

            identifiers.insert(
                reference.clone(),
                GTPModuleIdentifierResolve {
                    source: GTPModuleIdentifierSource::External(local_path.clone()),
                },
            );
        }

        Ok(GTPModuleResolve {
            paths,
            identifiers,
            definitions: Default::default(),
        })
    }
}
