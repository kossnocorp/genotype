use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PyProjectModule {
    module: PyModule,
}

impl GtlProjectModule for PyProjectModule {
    type LangConfig = PyConfig;
    type Module<'context> = PyModule;

    fn module(&self) -> &Self::Module<'_> {
        &self.module
    }

    fn convert(
        lang_config: &PyConfig,
        resolved: &GtpModuleResolved,
    ) -> Result<PyProjectModule, Box<dyn GtlError>> {
        let mut convert_resolve = PyConvertResolve::default();
        let mut prefixes: IndexMap<String, u8> = IndexMap::new();
        let parse = &resolved.project_module_parse.module_parse;
        let module_resolve = &resolved.resolve;

        // TODO: I'm pretty sure I can extract it and share with TypeScript
        for import in parse.module.imports.iter() {
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

        let module = PyModule::convert(&parse.module, &convert_resolve, lang_config);
        Ok(PyProjectModule { module })
    }

    fn new_render_context<'config>(
        &self,
        lang_config: &'config Self::LangConfig,
    ) -> PyRenderContext<'config> {
        PyRenderContext::new(&lang_config.lang)
    }
}
