use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct PyProjectModule {
    pub name: String,
    pub path: GtpPkgSrcDirRelativePath,
    pub module: PyModule,
}

impl GtlProjectModule<PyConfig> for PyProjectModule {
    type Dependency = PyDependencyIdent;

    fn generate(config: &PyConfig, module: &GtpModule) -> Result<Self> {
        let path = module.path.to_pkg_src_relative_path("py");
        let name = py_parse_module_path(path.with_extension("").as_str().into());

        let mut resolve = PyConvertResolve::default();
        let mut prefixes: HashMap<String, u8> = HashMap::new();

        // [TODO] I'm pretty sure I can extract it and share with TypeScript
        for import in module.module.imports.iter() {
            match &import.reference {
                GtImportReference::Glob(_) => {
                    let references = module
                        .resolve
                        .identifiers
                        .iter()
                        .filter(|(_, resolve)| {
                            if let GtpModuleIdentifierSource::External(path) = &resolve.source {
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

                        resolve.globs.insert(import.path.clone(), prefix.clone());

                        references.iter().for_each(|(reference, _)| {
                            let identifier = (*reference).clone();
                            let span = identifier.0;
                            let alias_str = format!("{}.{}", prefix, identifier.1);
                            let alias = GtIdentifier::new(span, alias_str.into());
                            resolve.identifiers.insert(identifier.clone(), alias);
                            resolve.imported.insert(identifier);
                        });
                    }
                }

                GtImportReference::Name(_, identifier) => {
                    resolve.imported.insert(identifier.clone());
                }

                GtImportReference::Names(_, identifiers) => {
                    identifiers.iter().for_each(|name| {
                        resolve.imported.insert(
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

        let module = PyConvertModule::convert(&module.module, &resolve, config).0;

        Ok(Self { name, path, module })
    }

    fn dependencies(&self) -> Vec<Self::Dependency> {
        self.module
            .imports
            .iter()
            .map(|import| import.dependency.clone())
            .collect()
    }
}

impl Hash for PyProjectModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
