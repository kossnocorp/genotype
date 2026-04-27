use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum TsProjectModule {
    Generated(TsProjectModuleGenerated),
    Error(TsProjectModuleError),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct TsProjectModuleGenerated {
    pub path: GtpPkgSrcDirRelativePath,
    pub module: TsModule,
    pub mode: TsMode,
}

#[derive(Error, Debug, PartialEq, Clone, Serialize)]
pub enum TsProjectModuleError {
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

impl GtlProjectModule<TsConfig> for TsProjectModule {
    type Dependency = TsDependencyIdent;

    fn generate(
        src_path: &GtpSrcDirPath,
        config: &TsConfig,
        module_path: &GtpModulePath,
        module: &GtpModule,
    ) -> Self {
        let resolved = match module {
            GtpModule::Resolved(module) => module,

            GtpModule::Initialized => {
                return Self::Error(TsProjectModuleError::Initialized {
                    path: module_path.clone(),
                });
            }

            GtpModule::Parsed(module) => {
                return Self::Error(TsProjectModuleError::Parsed {
                    path: module_path.clone(),
                    module_id: module.module_parse.module.id.clone(),
                });
            }

            GtpModule::Error(error) => {
                return Self::Error(TsProjectModuleError::ProjectModuleError {
                    path: module_path.clone(),
                    message: error.to_string(),
                });
            }
        };

        let path = module_path
            .to_module_id(src_path)
            .map(|module_id| {
                GtpPkgSrcDirRelativePath::from_str(&format!("{}.ts", module_id.0.as_ref()))
            })
            .unwrap_or_else(|_| GtpPkgSrcDirRelativePath::from_str("unknown.ts"));

        let mut resolve = TsConvertResolve::new();
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

                    resolve.globs.insert(import.path.clone(), prefix.clone());

                    references.iter().for_each(|(reference, _)| {
                        let identifier = (*reference).clone();
                        let span = identifier.0;
                        let alias = format!("{}.{}", prefix, identifier.1);
                        resolve
                            .identifiers
                            .insert(identifier, GtIdentifier::new(span, alias.into()));
                    });
                }
            }
        }

        let module = TsConvertModule::convert(&parse.module, resolve, config).0;

        Self::Generated(TsProjectModuleGenerated {
            path,
            module,
            mode: config.lang.mode.clone(),
        })
    }

    fn dependencies(&self) -> Vec<Self::Dependency> {
        match self {
            Self::Generated(module) => {
                if module.mode == TsMode::Zod {
                    vec![TsDependencyIdent::Zod]
                } else {
                    vec![]
                }
            }
            Self::Error(_) => vec![],
        }
    }
}

impl Hash for TsProjectModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Generated(module) => module.path.hash(state),
            Self::Error(error) => match error {
                TsProjectModuleError::Initialized { path }
                | TsProjectModuleError::Parsed { path, .. }
                | TsProjectModuleError::ProjectModuleError { path, .. } => path.hash(state),
            },
        }
    }
}
