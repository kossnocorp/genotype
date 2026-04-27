use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct PyProjectModuleGenerated {
    pub name: String,
    pub path: GtpPkgSrcDirRelativePath,
    pub module: PyModule,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum PyProjectModule {
    Generated(PyProjectModuleGenerated),
    Error(PyProjectModuleError),
}

#[derive(Error, Debug, PartialEq, Clone, Serialize)]
pub enum PyProjectModuleError {
    #[error("Can't generate module `{path}` because it is still in initialized state")]
    Initialized { path: GtpModulePath },

    #[error(
        "Can't generate module `{path}` because it is still in parsed state with id `{module_id:?}`"
    )]
    Parsed {
        path: GtpModulePath,
        module_id: GtModuleId,
    },

    #[error("Project module `{path}` error: {message}")]
    ProjectModuleError {
        path: GtpModulePath,
        message: String,
    },
}

impl GtlProjectModule<PyConfig> for PyProjectModule {
    type Dependency = PyDependencyIdent;

    fn generate(
        src_path: &GtpSrcDirPath,
        config: &PyConfig,
        module_path: &GtpModulePath,
        module: &GtpModule,
    ) -> Self {
        let resolved = match module {
            GtpModule::Resolved(module) => module,
            GtpModule::Initialized => {
                return Self::Error(PyProjectModuleError::Initialized {
                    path: module_path.clone(),
                });
            }
            GtpModule::Parsed(module) => {
                return Self::Error(PyProjectModuleError::Parsed {
                    path: module_path.clone(),
                    module_id: module.module_parse.module.id.clone(),
                });
            }
            GtpModule::Error(error) => {
                return Self::Error(PyProjectModuleError::ProjectModuleError {
                    path: module_path.clone(),
                    message: error.to_string(),
                });
            }
        };

        let path = module_path
            .to_module_id(&src_path)
            .map(|module_id| {
                GtpPkgSrcDirRelativePath::from_str(&format!("{}.py", module_id.0.as_ref()))
            })
            .unwrap_or_else(|_| GtpPkgSrcDirRelativePath::from_str("unknown.py"));
        let name = py_parse_module_path(path.with_extension("").as_str().into());

        let mut resolve = PyConvertResolve::default();
        let mut prefixes: IndexMap<String, u8> = IndexMap::new();
        let parse = &resolved.project_module_parse.module_parse;
        let module_resolve = &resolved.resolve;

        // [TODO] I'm pretty sure I can extract it and share with TypeScript
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

        let module = PyConvertModule::convert(&parse.module, &resolve, config).0;

        Self::Generated(PyProjectModuleGenerated { name, path, module })
    }

    fn dependencies(&self) -> Vec<Self::Dependency> {
        match self {
            Self::Generated(module) => module
                .module
                .imports
                .iter()
                .map(|import| import.dependency.clone())
                .collect(),
            Self::Error(_) => vec![],
        }
    }
}

impl Hash for PyProjectModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Generated(module) => module.path.hash(state),
            Self::Error(error) => match error {
                PyProjectModuleError::Initialized { path }
                | PyProjectModuleError::Parsed { path, .. }
                | PyProjectModuleError::ProjectModuleError { path, .. } => path.hash(state),
            },
        }
    }
}
