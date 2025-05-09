use crate::prelude::internal::*;

impl RsProject<'_> {
    pub fn modules_source(&self) -> Result<Vec<GtlProjectFile>> {
        let mut context = RSRenderContext {
            config: &self.config.target.lang,
        };
        self.modules
            .iter()
            .map(
                |module| match module.module.render(Default::default(), &mut context) {
                    Ok(code) => Ok(GtlProjectFile {
                        path: self.config.pkg_src_file_path(&module.path),
                        source: code,
                    }),
                    Err(err) => Err(err),
                },
            )
            .collect::<Result<Vec<_>>>()
    }

    pub fn generate_modules(
        config: &RsConfig,
        modules: &Vec<GtProjectModule>,
    ) -> Result<Vec<RsProjectModule>> {
        let mut modules = modules
            .iter()
            .map(|module| RsProjectModule::generate(&config, module))
            .collect::<Result<Vec<RsProjectModule>, _>>()?;

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
                Some((current_definition_id, (span, reference_ids))) => {
                    let mut fields = vec![];

                    for id in reference_ids {
                        let reference_fields = modules
                            .iter()
                            .flat_map(|module| module.module.definitions.iter())
                            .find(|definition| definition.id() == id)
                            .ok_or_else(|| {
                                RSProjectError::BuildModulePath(format!(
                                    "Failed to find reference with id {module_id}/{id}",
                                    module_id = current_definition_id.0 .0,
                                    id = current_definition_id.1
                                ))
                            })
                            .and_then(|reference| match reference {
                                RSDefinition::Struct(reference_struct) => {
                                    match &reference_struct.fields {
                                        RSStructFields::Resolved(resolved_fields) => {
                                            Ok(resolved_fields.clone())
                                        }

                                        RSStructFields::Newtype(_) => {
                                            Err(RSProjectError::TupleStructExtension(span.clone())
                                                .into())
                                        }

                                        RSStructFields::Unit => {
                                            Err(RSProjectError::UnitStructExtension(span.clone())
                                                .into())
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

                    // Collect the nested extension field references.
                    let mut visitor = RSProjectExtensionFieldsVisitor::new();
                    for field in fields.iter_mut() {
                        field.traverse(&mut visitor);
                    }

                    let module = modules
                        .iter_mut()
                        .find(|module| module.module.id == current_definition_id.0)
                        .ok_or_else(|| {
                            RSProjectError::FailedExtensionsResolve(
                                span.clone(),
                                format!(
                                    "Can't find module with id {id}",
                                    id = current_definition_id.0 .0
                                ),
                            )
                        })?;

                    let cleared_references = module
                        .module
                        .definitions
                        .iter_mut()
                        .find(|definition| definition.id() == current_definition_id)
                        .ok_or_else(|| {
                            RSProjectError::FailedExtensionsResolve(
                                span.clone(),
                                format!(
                                    "Can't find definition {module_id}/{id}",
                                    module_id = current_definition_id.0 .0,
                                    id = current_definition_id.1
                                ),
                            )
                        })
                        .and_then(|definition| {
                            if let RSDefinition::Struct(r#struct) = definition {
                                match &r#struct.fields {
                                    RSStructFields::Unresolved(_, references, own_fields) => {
                                        let references = references.clone();
                                        fields.extend(own_fields.clone());
                                        r#struct.fields = RSStructFields::Resolved(fields);
                                        Ok(references)
                                    }

                                    _ => Err(RSProjectError::FailedExtensionsResolve(
                                        span.clone(),
                                        "Definition is already resolved".into(),
                                    )),
                                }
                            } else {
                                Err(RSProjectError::FailedExtensionsResolve(
                                    span.clone(),
                                    "Definition is not a struct".into(),
                                ))
                            }
                        })?;

                    // Remove the extension references from the map, so we can optimize uses later.
                    for reference in cleared_references {
                        module
                            .resolve
                            .definitions
                            .entry(reference.definition_id)
                            .and_modify(|resolve| {
                                resolve.references.remove(&reference.id);
                            });
                    }

                    // Now add references pulled from the extension fields.
                    for (reference_id, definition_id) in visitor.references.clone() {
                        let resolve = module
                            .resolve
                            .definitions
                            .entry(definition_id)
                            .or_insert_with(Default::default);
                        resolve.references.insert(reference_id);
                    }

                    // Add missing uses from the extension fields.
                    for (_, reference_definition_id) in visitor.references {
                        if reference_definition_id.0 == current_definition_id.0 {
                            continue;
                        }

                        let existing_use = module.module.imports.iter_mut().find(|import| {
                            if let RSDependencyIdent::Local(path) = &import.dependency {
                                return path.0 == reference_definition_id.0;
                            }
                            false
                        });

                        match existing_use {
                            Some(r#use) => match &r#use.reference {
                                RSUseReference::Module => {
                                    todo!("Rewrite the copied dependency to module import");
                                }

                                RSUseReference::Named(names) => {
                                    let mut names = names.clone();
                                    // [TODO] Pass through Rust renamer?
                                    names.push(RSUseName::Name(
                                        reference_definition_id.1.clone().into(),
                                    ));
                                    r#use.reference = RSUseReference::Named(names);
                                }

                                // Nothing to do, glob import is already there
                                RSUseReference::Glob => {}
                            },

                            None => {
                                // Create new named import
                                module.module.imports.push(RSUse {
                                    reference: RSUseReference::Named(vec![RSUseName::Name(
                                        reference_definition_id.1.clone().into(),
                                    )]),
                                    dependency: RSDependencyIdent::Local(RSPath(
                                        reference_definition_id.0.clone(),
                                        format!("crate::{}", reference_definition_id.0 .0).into(),
                                    )),
                                });
                            }
                        }
                    }

                    // Remove all unused imports
                    module
                        .module
                        .imports
                        .retain(|r#use| match &r#use.dependency {
                            RSDependencyIdent::Local(path) => {
                                module.resolve.definitions.values().any(|resolve| {
                                    resolve
                                        .references
                                        .iter()
                                        .any(|reference| reference.0 == path.0)
                                })
                            }
                            _ => true,
                        });

                    // Clean up references
                    module.module.imports.iter_mut().for_each(|r#use| {
                        if let RSDependencyIdent::Local(path) = &r#use.dependency {
                            if let RSUseReference::Named(names) = &r#use.reference {
                                let mut names = names.clone();
                                names.retain(|name| {
                                    let resolve = module.resolve.definitions.get(
                                        // [TODO] Avoid creating this in-place
                                        &GTDefinitionId(path.0.clone(), name.name().clone().0),
                                    );

                                    if let Some(resolve) = resolve {
                                        return resolve.references.len() > 0;
                                    }

                                    false
                                });
                                r#use.reference = RSUseReference::Named(names);
                            }
                        }
                    });

                    current_definition_id.clone()
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

struct RSProjectExtensionFieldsVisitor {
    references: IndexSet<(GTReferenceId, GTDefinitionId)>,
}

impl RSProjectExtensionFieldsVisitor {
    fn new() -> Self {
        Self {
            references: Default::default(),
        }
    }
}

impl RSVisitor for RSProjectExtensionFieldsVisitor {
    fn visit_reference(&mut self, reference: &mut RSReference) {
        self.references
            .insert((reference.id.clone(), reference.definition_id.clone()));

        // Restore the identifier in case it was renamed to include the module name.
        // [TODO] Pass through Rust renamer?
        reference.identifier = reference.definition_id.1.clone().into();
    }
}
