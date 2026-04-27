use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct RsProjectModuleGenerated {
    pub path: GtpPkgSrcDirRelativePath,
    pub module: RsModule,
    pub resolve: RspModuleResolve,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum RsProjectModule {
    Generated(RsProjectModuleGenerated),
    Error(RsProjectModuleError),
}

#[derive(Error, Debug, PartialEq, Clone, Serialize)]
pub enum RsProjectModuleError {
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

    #[error("Failed generating module `{path}`: {message}")]
    Generate {
        path: GtpModulePath,
        message: String,
    },
}

impl GtlProjectModule<RsConfig> for RsProjectModule {
    type Dependency = RsDependencyIdent;

    fn generate(
        src_path: &GtpSrcDirPath,
        config: &RsConfig,
        module_path: &GtpModulePath,
        module: &GtpModule,
    ) -> Self {
        let resolved = match module {
            GtpModule::Resolved(module) => module,
            GtpModule::Initialized => {
                return Self::Error(RsProjectModuleError::Initialized {
                    path: module_path.clone(),
                });
            }
            GtpModule::Parsed(module) => {
                return Self::Error(RsProjectModuleError::Parsed {
                    path: module_path.clone(),
                    module_id: module.module_parse.module.id.clone(),
                });
            }
            GtpModule::Error(error) => {
                return Self::Error(RsProjectModuleError::ProjectModuleError {
                    path: module_path.clone(),
                    message: error.to_string(),
                });
            }
        };

        let path = module_path
            .to_module_id(src_path)
            .map(|module_id| {
                GtpPkgSrcDirRelativePath::from_str(&format!("{}.rs", module_id.0.as_ref()))
            })
            .unwrap_or_else(|_| {
                resolved
                    .project_module_parse
                    .path
                    .cwd_relative_path()
                    .relative_path()
                    .strip_prefix("src")
                    .map(|relative| GtpPkgSrcDirRelativePath::new(relative.with_extension("rs")))
                    .unwrap_or_else(|_| GtpPkgSrcDirRelativePath::from_str("unknown.rs"))
            });

        let mut convert_resolve = RsConvertResolve::default();
        let mut prefixes: IndexMap<String, u8> = IndexMap::new();
        let parse = &resolved.project_module_parse.module_parse;
        let module_resolve = &resolved.resolve;

        // [TODO] I'm pretty sure I can extract it and share with TypeScript and Python too
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

        let definitions = module_resolve.definitions.clone();
        let resolve = RspModuleResolve { definitions };

        let module = match RsConvertModule::convert(&parse.module, &convert_resolve, config)
            .map_err(|err| err.with_source_code(resolved.project_module_parse.source_code.clone()))
        {
            Ok(module) => module.0,
            Err(err) => {
                return Self::Error(RsProjectModuleError::Generate {
                    path: module_path.clone(),
                    message: err.to_string(),
                });
            }
        };

        Self::Generated(RsProjectModuleGenerated {
            path,
            module,
            resolve,
        })
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

impl Hash for RsProjectModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Generated(module) => module.path.hash(state),
            Self::Error(error) => match error {
                RsProjectModuleError::Initialized { path }
                | RsProjectModuleError::Parsed { path, .. }
                | RsProjectModuleError::ProjectModuleError { path, .. }
                | RsProjectModuleError::Generate { path, .. } => path.hash(state),
            },
        }
    }
}
