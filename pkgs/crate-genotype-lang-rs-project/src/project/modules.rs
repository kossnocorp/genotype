use crate::prelude::internal::*;

impl RsProject<'_> {
    pub fn modules_source(&self) -> Result<Vec<GtlProjectFile>> {
        let mut context = RsRenderContext {
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
        let mut project_modules = modules
            .iter()
            .map(|module| RsProjectModule::generate(&config, module))
            .collect::<Result<Vec<RsProjectModule>, _>>()?;

        // Now when we generated modules, we need to go through all structs and resolve the fields.
        // Whenever extension is found, we have to copy the fields from the referenced struct as
        // Rust doesn't support inheritance in any acceptable way.

        let mut definitions_to_resolve: IndexMap<
            GtDefinitionId,
            (GtSpan, IndexSet<GtDefinitionId>),
        > = Default::default();

        // First, we collect all the definitions in all the modules that need to be resolved with
        // their extensions.
        for project_module in project_modules.iter() {
            for definition in project_module.module.definitions.iter() {
                if let RsDefinition::Struct(r#struct) = definition {
                    if let RsStructFields::Unresolved(span, references, _) = &r#struct.fields {
                        let reference_ids = references
                            .iter()
                            .map(|reference| reference.definition_id.clone())
                            .collect::<IndexSet<_>>();
                        definitions_to_resolve
                            .insert(r#struct.id.clone(), (span.clone(), reference_ids));
                    }
                }
            }
        }

        // Iterate over the definitions until we resolve them all.
        loop {
            if definitions_to_resolve.is_empty() {
                break;
            }

            // Now we start resolving the definition extensions starting from the ones that don't
            // reference any other definitions in the map.
            let definition = definitions_to_resolve
                .iter()
                .find(|(_, (_, referenced_ids))| {
                    referenced_ids
                        .iter()
                        .all(|referenced_id| !definitions_to_resolve.contains_key(referenced_id))
                });

            let resolved_definition_id = match definition {
                Some((current_definition_id, (span, referenced_ids))) => {
                    let mut resolved_fields = vec![];

                    // Collect fields for each referenced definition.
                    for referenced_id in referenced_ids {
                        let referenced_fields = project_modules
                            .iter()
                            .flat_map(|project_module| project_module.module.definitions.iter())
                            .find(|definition| definition.id() == referenced_id)
                            .ok_or_else(|| {
                                RsProjectError::BuildModulePath(format!(
                                    "Failed to find reference with id {module_id}/{referenced_id}",
                                    module_id = current_definition_id.0 .0,
                                    referenced_id = current_definition_id.1
                                ))
                            })
                            .and_then(|reference| match reference {
                                RsDefinition::Struct(reference_struct) => {
                                    match &reference_struct.fields {
                                        RsStructFields::Resolved(resolved_fields) => {
                                            Ok(resolved_fields.clone())
                                        }

                                        RsStructFields::Newtype(_) => {
                                            Err(RsProjectError::TupleStructExtension(span.clone()))
                                        }

                                        RsStructFields::Unit => {
                                            Err(RsProjectError::UnitStructExtension(span.clone()))
                                        }

                                        RsStructFields::Unresolved(span, _, _) => {
                                            // [TODO] Include the current struct too into the error.
                                            Err(RsProjectError::FailedExtensionsResolve(
                                                span.clone(),
                                                "Referenced extension is not resolved".into(),
                                            ))
                                        }
                                    }
                                }

                                // [TODO] It should be possible to extend aliases too as long as
                                // they reference other structs. Enums too but they must be handled
                                // differently.
                                _ => Err(RsProjectError::NonStructExtension(
                                    span.clone(),
                                    reference.name().0.to_string(),
                                )
                                .into()),
                            })?;

                        resolved_fields.extend(referenced_fields);
                    }

                    // Collect the referenced definition field references.
                    let mut visitor = RsProjectExtensionFieldsVisitor::new();
                    for field in resolved_fields.iter_mut() {
                        field.traverse(&mut visitor);
                    }

                    // Find the module that contains the current definition id.
                    let module = project_modules
                        .iter_mut()
                        .find(|module| module.module.id == current_definition_id.0)
                        .ok_or_else(|| {
                            RsProjectError::FailedExtensionsResolve(
                                span.clone(),
                                format!(
                                    "Can't find module with id {id}",
                                    id = current_definition_id.0 .0
                                ),
                            )
                        })?;

                    // Resolve the current definition fields using the collected extension fields.
                    let cleared_extension_references = module
                        .module
                        .definitions
                        .iter_mut()
                        .find(|definition| definition.id() == current_definition_id)
                        .ok_or_else(|| {
                            RsProjectError::FailedExtensionsResolve(
                                span.clone(),
                                format!(
                                    "Can't find definition {module_id}/{id}",
                                    module_id = current_definition_id.0 .0,
                                    id = current_definition_id.1
                                ),
                            )
                        })
                        .and_then(|definition| {
                            if let RsDefinition::Struct(r#struct) = definition {
                                match &r#struct.fields {
                                    RsStructFields::Unresolved(_, references, own_fields) => {
                                        let extension_references = references.clone();

                                        // [MARK] Here is where we copy the fields from all
                                        // the extensions into the current struct.
                                        resolved_fields.extend(own_fields.clone());
                                        r#struct.fields = RsStructFields::Resolved(resolved_fields);
                                        // Return the references to the extension fields, so we can
                                        // remove them

                                        Ok(extension_references)
                                    }

                                    _ => Err(RsProjectError::FailedExtensionsResolve(
                                        span.clone(),
                                        "Definition is already resolved".into(),
                                    )),
                                }
                            } else {
                                Err(RsProjectError::FailedExtensionsResolve(
                                    span.clone(),
                                    "Definition is not a struct".into(),
                                ))
                            }
                        })?;

                    // Update resolve after the fields are copied.

                    // Remove the extension references from the resolve so we can optimize uses
                    // based on the remaining references.
                    for extension_reference in cleared_extension_references {
                        module
                            .resolve
                            .definitions
                            .entry(extension_reference.definition_id)
                            .and_modify(|resolve| {
                                resolve.references.remove(&extension_reference.id);
                            });
                    }

                    // Update the module resolve and add definition references.
                    for (reference_id, definition_id) in visitor.pulled_references.clone() {
                        let pulled_definition_resolve = module
                            .resolve
                            .definitions
                            .entry(definition_id)
                            .or_insert_with(Default::default);
                        pulled_definition_resolve.references.insert(reference_id);
                    }

                    // Add missing uses from the extension fields.
                    for (_, referenced_definition_id) in visitor.pulled_references {
                        // If pulled reference is from the same module, we don't need to import it.
                        if referenced_definition_id.0 == current_definition_id.0 {
                            continue;
                        }

                        // Try finding existing use that imports the referenced definition.
                        let existing_use = module.module.imports.iter_mut().find(|import| {
                            if let RsDependencyIdent::Local(path) = &import.dependency {
                                return path.0 == referenced_definition_id.0;
                            }
                            false
                        });

                        match existing_use {
                            // There's a use.
                            Some(r#use) => match &r#use.reference {
                                // If that's a module use, we need to rewrite the existing
                                // references.
                                RsUseReference::Module => {
                                    todo!(
                                        "Rewrite the copied extension references to the module import"
                                    );
                                }

                                // If that's a named use, we need to add the referenced name to
                                // the list.
                                RsUseReference::Named(names) => {
                                    // If the name is already in the list, we don't need to
                                    // add it again.
                                    let already_imported = names.iter().any(|use_name| {
                                        use_name.name().0 == referenced_definition_id.1
                                    });
                                    if !already_imported {
                                        let mut names = names.clone();
                                        // [TODO] Pass through Rust renamer?
                                        names.push(RsUseName::Name(
                                            referenced_definition_id.1.clone().into(),
                                        ));
                                        r#use.reference = RsUseReference::Named(names);
                                    }
                                }

                                // If that's a glob, there's nothing to do as it imports everything.
                                RsUseReference::Glob => {}
                            },

                            // If there's none, add a new use.
                            None => {
                                // Create new named import
                                module.module.imports.push(RsUse {
                                    reference: RsUseReference::Named(vec![RsUseName::Name(
                                        referenced_definition_id.1.clone().into(),
                                    )]),
                                    dependency: RsDependencyIdent::Local(RsPath(
                                        referenced_definition_id.0.clone(),
                                        format!("crate::{}", referenced_definition_id.0 .0).into(),
                                    )),
                                });
                            }
                        }
                    }

                    // Clean up references
                    module.module.imports.iter_mut().for_each(|r#use| {
                        if let RsDependencyIdent::Local(path) = &r#use.dependency {
                            if let RsUseReference::Named(names) = &r#use.reference {
                                let mut names = names.clone();
                                names.retain(|name| {
                                    let resolve = module.resolve.definitions.get(
                                        // [TODO] Avoid creating this in-place
                                        &GtDefinitionId(path.0.clone(), name.name().clone().0),
                                    );

                                    if let Some(resolve) = resolve {
                                        return resolve.references.len() > 0;
                                    }

                                    false
                                });
                                r#use.reference = RsUseReference::Named(names);
                            }
                        }
                    });

                    current_definition_id.clone()
                }

                // Found no definition that doesn't reference any other definition.
                None => {
                    return Err(RsProjectError::CyclicExtensions(
                        definitions_to_resolve
                            .iter()
                            .map(|(_, (span, _))| span.clone())
                            .collect(),
                    ))
                    .into_diagnostic();
                }
            };

            // We delay the removal of the definition iid from the map to avoid borrowing issues.
            definitions_to_resolve.shift_remove(&resolved_definition_id);
        }

        // Remove all unused imports
        for project_module in project_modules.iter_mut() {
            project_module
                .module
                .imports
                .retain(|r#use| match &r#use.dependency {
                    // Only process local dependencies for now
                    RsDependencyIdent::Local(path) => {
                        project_module
                            .resolve
                            .definitions
                            .iter()
                            .any(|(definition_id, resolve)| {
                                // If there are any references to the definition, we keep the use.
                                definition_id.0 == path.0 && !resolve.references.is_empty()
                            })
                    }

                    // [TODO] Process external dependencies too.
                    _ => true,
                });
        }

        Ok(project_modules)
    }
}

struct RsProjectExtensionFieldsVisitor {
    /// References pulled from the extension fields while traversing.
    pulled_references: IndexSet<(GtReferenceId, GtDefinitionId)>,
}

impl RsProjectExtensionFieldsVisitor {
    fn new() -> Self {
        Self {
            pulled_references: Default::default(),
        }
    }
}

impl RsVisitor for RsProjectExtensionFieldsVisitor {
    fn visit_reference(&mut self, reference: &mut RsReference) {
        self.pulled_references
            .insert((reference.id.clone(), reference.definition_id.clone()));

        // Restore the identifier in case it was renamed to include the module name.
        // [TODO] Pass through Rust renamer?
        reference.identifier = reference.definition_id.1.clone().into();
    }
}
