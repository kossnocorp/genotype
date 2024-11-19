use genotype_lang_core_project::{module::GTLangProjectModule, source::GTLangProjectSource};
use genotype_lang_rs_config::RSProjectConfig;
use genotype_lang_rs_tree::{rs_indent, RSDefinition, RSRender, RSStructFields};
use genotype_parser::{GTDefinitionId, GTSpan};
use genotype_project::GTProject;
use indexmap::{IndexMap, IndexSet};
use miette::Result;

use crate::{error::RSProjectError, module::RSProjectModule};

use super::RSProject;

impl RSProject {
    pub fn modules_source(&self) -> Result<Vec<GTLangProjectSource>> {
        self.modules
            .iter()
            .map(
                |module| match module.module.render(&rs_indent(), &self.config.lang) {
                    Ok(code) => Ok(GTLangProjectSource {
                        path: module.path.clone(),
                        code,
                    }),
                    Err(err) => Err(err),
                },
            )
            .collect::<Result<Vec<_>>>()
    }

    pub fn generate_modules(
        project: &GTProject,
        config: &RSProjectConfig,
    ) -> Result<Vec<RSProjectModule>> {
        let mut modules = project
            .modules
            .iter()
            .map(|module| RSProjectModule::generate(&project, module, &config))
            .collect::<Result<Vec<RSProjectModule>, _>>()?;

        // Now when we generated modules, we need to go through all structs and resolve their fields
        // by copying the fields from the referenced struct as Rust doesn't support inheritance in
        // any acceptable way.

        let mut to_resolve: IndexMap<GTDefinitionId, (GTSpan, IndexSet<GTDefinitionId>)> =
            Default::default();

        // First, we collect all the definitions that need to be resolved with their extensions
        for module in modules.iter() {
            for definition in module.module.definitions.iter() {
                if let RSDefinition::Struct(r#struct) = definition {
                    if let RSStructFields::Unresolved(span, references, _) = &r#struct.fields {
                        let reference_ids = references
                            .iter()
                            .map(|reference| reference.definition_id.clone())
                            .collect::<IndexSet<_>>();
                        to_resolve.insert(r#struct.id.clone(), (span.clone(), reference_ids));
                    }
                }
            }
        }

        // Now we start resolving the definition extensions by looking ones that don't reference
        // any other definitions in the map.
        loop {
            if to_resolve.is_empty() {
                break;
            }

            let definition = to_resolve.iter().find(|(_, (_, reference_ids))| {
                reference_ids
                    .iter()
                    .all(|reference_id| !to_resolve.contains_key(reference_id))
            });

            let to_remove = match definition {
                Some((definition_id, (span, reference_ids))) => {
                    let mut fields = vec![];

                    for id in reference_ids {
                        let reference_fields = modules
                            .iter()
                            .flat_map(|module| module.module.definitions.iter())
                            .find(|definition| definition.id() == id)
                            .ok_or_else(|| {
                                RSProjectError::BuildModulePath(format!(
                                    "Failed to find reference with id {module_id}/{id}",
                                    module_id = definition_id.0 .0,
                                    id = definition_id.1
                                ))
                            })
                            .and_then(|reference| match reference {
                                RSDefinition::Struct(reference_struct) => {
                                    match &reference_struct.fields {
                                        RSStructFields::Resolved(resolved_fields) => {
                                            Ok(resolved_fields.clone())
                                        }

                                        RSStructFields::Unresolved(span, _, _) => {
                                            // [TODO] Include the current struct too
                                            Err(RSProjectError::FailedExtensionsResolve(
                                                span.clone(),
                                                "Referenced extension is not resolved".into(),
                                            ))
                                        }
                                    }
                                }

                                _ => Err(RSProjectError::NonStructExtension(
                                    span.clone(),
                                    reference.name().0.clone(),
                                )
                                .into()),
                            })?;

                        fields.extend(reference_fields);
                    }

                    modules
                        .iter_mut()
                        .find(|module| module.module.id == definition_id.0)
                        .ok_or_else(|| {
                            RSProjectError::FailedExtensionsResolve(
                                span.clone(),
                                format!("Can't find module with id {id}", id = definition_id.0 .0),
                            )
                        })
                        .and_then(|module| {
                            module
                                .module
                                .definitions
                                .iter_mut()
                                .find(|definition| definition.id() == definition_id)
                                .ok_or_else(|| {
                                    RSProjectError::FailedExtensionsResolve(
                                        span.clone(),
                                        format!(
                                            "Can't find definition {module_id}/{id}",
                                            module_id = definition_id.0 .0,
                                            id = definition_id.1
                                        ),
                                    )
                                })
                        })
                        .and_then(|definition| {
                            if let RSDefinition::Struct(r#struct) = definition {
                                r#struct.fields = RSStructFields::Resolved(fields);
                                Ok(())
                            } else {
                                Err(RSProjectError::FailedExtensionsResolve(
                                    span.clone(),
                                    "Definition is not a struct".into(),
                                ))
                            }
                        })?;

                    definition_id.clone()
                }

                None => {
                    return Err(RSProjectError::CyclicExtensions(
                        to_resolve
                            .iter()
                            .map(|(_, (span, _))| span.clone())
                            .collect(),
                    )
                    .into())
                }
            };

            // We delay the removal of the definition from the map to avoid borrowing issues
            to_resolve.shift_remove(&to_remove);
        }

        Ok(modules)
    }
}
