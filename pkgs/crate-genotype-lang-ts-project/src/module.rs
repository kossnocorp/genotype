use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TsProjectModule {
    module: TsModule,
}

impl GtlProjectModule for TsProjectModule {
    type LangConfig = TsConfig;
    type Module<'context> = TsModule;

    fn module(&self) -> &Self::Module<'_> {
        &self.module
    }

    fn convert(
        lang_config: &TsConfig,
        resolved: &GtpModuleResolved,
    ) -> Result<TsProjectModule, Box<dyn GtlError>> {
        let mut convert_resolve = TsConvertResolve::new();
        let mut prefixes: IndexMap<String, u8> = IndexMap::new();
        let parse = &resolved.project_module_parse.module_parse;
        let module_resolve = &resolved.resolve;

        for import in parse.module.imports.iter() {
            if let GtImportReference::Glob(_) = import.reference {
                let references = module_resolve
                    .identifiers
                    .iter()
                    .filter(|(_, resolve)| {
                        if let GtpModuleResolveIdentifierSource::External(path) = &resolve.source {
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
                        let alias = format!("{}.{}", prefix, identifier.1);
                        convert_resolve
                            .identifiers
                            .insert(identifier, GtIdentifier::new(span, alias.into()));
                    });
                }
            }
        }

        let module = TsModule::convert(&parse.module, convert_resolve, lang_config);
        Ok(TsProjectModule { module })
    }

    fn new_render_context<'config>(
        &self,
        lang_config: &'config Self::LangConfig,
    ) -> TsRenderContext<'config> {
        TsRenderContext::new(&lang_config.lang)
    }

    fn global_dependencies(lang_config: &TsConfig) -> Option<IndexSet<TsDependencyIdent>> {
        match lang_config.lang.mode {
            TsMode::Zod => Some(IndexSet::from_iter(vec![TsDependencyIdent::Zod])),
            _ => None,
        }
    }
}
