use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone)]
pub struct RsProjectModule {
    pub path: GtPkgSrcRelativePath,
    pub module: RSModule,
    pub resolve: RSPModuleResolve,
}

impl GtlProjectModule<RsConfig> for RsProjectModule {
    type Dependency = RSDependencyIdent;

    fn generate(config: &RsConfig, module: &GtProjectModule) -> Result<Self> {
        let path = module.path.to_pkg_src_relative_path("rs");

        let mut convert_resolve = RSConvertResolve::default();
        let mut prefixes: HashMap<String, u8> = HashMap::new();

        // [TODO] I'm pretty sure I can extract it and share with TypeScript and Python too
        for import in module.module.imports.iter() {
            match &import.reference {
                GTImportReference::Glob(_) => {
                    let references = module
                        .resolve
                        .identifiers
                        .iter()
                        .filter(|(_, resolve)| {
                            if let GTPModuleIdentifierSource::External(path) = &resolve.source {
                                return import.path == *path;
                            }
                            false
                        })
                        .collect::<Vec<_>>();

                    if references.len() > 0 {
                        let str = import.path.source_str();
                        let name = str.split('/').last().unwrap_or(str).to_string();
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
                            let span = identifier.0.clone();
                            let alias_str = format!("{}.{}", prefix, identifier.1);
                            let alias = GTIdentifier::new(span, alias_str);
                            convert_resolve
                                .identifiers
                                .insert(identifier.clone(), alias);
                            convert_resolve.imported.insert(identifier);
                        });
                    }
                }

                GTImportReference::Name(_, identifier) => {
                    convert_resolve.imported.insert(identifier.clone());
                }

                GTImportReference::Names(_, identifiers) => {
                    identifiers.iter().for_each(|name| {
                        convert_resolve.imported.insert(
                            match name {
                                GTImportName::Name(_, identifier) => identifier,
                                GTImportName::Alias(_, _, identifier) => identifier,
                            }
                            .clone(),
                        );
                    });
                }
            }
        }

        let definitions = module.resolve.definitions.clone();
        let resolve = RSPModuleResolve { definitions };

        let module = RSConvertModule::convert(&module.module, &convert_resolve, &config)
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
