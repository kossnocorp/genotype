use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct TsProjectModule {
    pub path: GtpPkgSrcDirRelativePath,
    pub module: TsModule,
    pub mode: TsMode,
}

impl GtlProjectModule<TsConfig> for TsProjectModule {
    type Dependency = TsDependencyIdent;

    fn generate(config: &TsConfig, module: &GtpModule) -> Result<Self> {
        let path = module.path.to_pkg_src_relative_path("ts");

        let mut resolve = TsConvertResolve::new();
        let mut prefixes: HashMap<String, u8> = HashMap::new();

        for import in module.module.imports.iter() {
            if let GtImportReference::Glob(_) = import.reference {
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
                        let alias = format!("{}.{}", prefix, identifier.1);
                        resolve
                            .identifiers
                            .insert(identifier, GtIdentifier::new(span, alias.into()));
                    });
                }
            }
        }

        let module = TsConvertModule::convert(&module.module, resolve, config).0;

        Ok(Self {
            path,
            module,
            mode: config.lang.mode.clone(),
        })
    }

    fn dependencies(&self) -> Vec<Self::Dependency> {
        if self.mode == TsMode::Zod {
            vec![TsDependencyIdent::Zod]
        } else {
            vec![]
        }
    }
}

impl Hash for TsProjectModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
