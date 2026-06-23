use crate::prelude::internal::*;

mod resolve;
pub use resolve::*;

mod extensions;

mod boxing;

mod tree_shaking;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RsProjectModule {
    module: RsModule,
    project_resolve: RsProjectModuleResolve,
}

impl GtlProjectModule for RsProjectModule {
    type LangConfig = RsConfig;
    type Module<'context> = RsModule;

    fn module(&self) -> &Self::Module<'_> {
        &self.module
    }

    fn convert(
        lang_config: &RsConfig,
        resolved: &GtpModuleResolved,
    ) -> Result<Self, Box<dyn GtlError>> {
        let mut convert_resolve = RsConvertResolve::default();
        let mut prefixes: IndexMap<String, u8> = IndexMap::new();
        let parse = &resolved.project_module_parse.module_parse;
        let module_resolve = &resolved.resolve;

        // TODO: I'm pretty sure I can extract it and share with TypeScript and Python too
        for import in parse.module.imports.iter() {
            if import.path.kind() == GtPathKind::Package {
                convert_resolve.path_module_ids.insert(
                    import.path.id.clone(),
                    GtModuleId(import.path.source_str().to_owned().into()),
                );
            }

            match &import.reference {
                GtImportReference::Glob(_) => {
                    let references = module_resolve
                        .identifiers
                        .iter()
                        .filter(|(_, resolve)| {
                            if let GtpModuleResolveIdentifierSource::External(path) =
                                &resolve.source
                            {
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

        module_resolve.paths.iter().for_each(|(path, module_path)| {
            convert_resolve
                .path_module_ids
                .insert(path.id.clone(), module_path.clone().into());
        });

        convert_resolve.reference_definition_ids = module_resolve
            .reference_definition_ids
            .iter()
            .map(|(reference_id, definition_id)| (reference_id.clone(), definition_id.clone()))
            .collect();

        let project_resolve = RsProjectModuleResolve::new(module_resolve);

        let module = RsModule::convert(&parse.module, &convert_resolve, lang_config)?;

        Ok(RsProjectModule {
            module,
            project_resolve,
        })
    }

    fn resolve_modules(
        _lang_config: &RsConfig,
        modules: &mut IndexMap<GtpModulePath, GtlProjectModuleState<RsProjectModule>>,
    ) -> Result<(), GtlProjectError> {
        Self::resolve_extensions(modules)?;
        Self::box_modules(modules);
        Self::tree_shake_modules(modules);
        Self::resolve_all_eligible_modules(modules);
        Ok(())
    }

    fn new_render_context<'config>(
        &self,
        lang_config: &'config Self::LangConfig,
    ) -> RsRenderContext<'config> {
        RsRenderContext::new(&lang_config.lang)
    }
}
