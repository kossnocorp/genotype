use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone)]
pub struct TsProjectModule {
    pub path: GtPkgSrcRelativePath,
    pub module: TSModule,
}

impl GtlProjectModule<TsConfig> for TsProjectModule {
    type Dependency = TSDependencyIdent;

    fn generate(config: &TsConfig, module: &GtProjectModule) -> Result<Self> {
        let path = module.path.to_pkg_src_relative_path("ts");

        let mut resolve = TSConvertResolve::new();
        let mut prefixes: HashMap<String, u8> = HashMap::new();

        for import in module.module.imports.iter() {
            if let GTImportReference::Glob(_) = import.reference {
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

                    resolve.globs.insert(import.path.clone(), prefix.clone());

                    references.iter().for_each(|(reference, _)| {
                        let identifier = (*reference).clone();
                        let span = identifier.0.clone();
                        let alias = format!("{}.{}", prefix, identifier.1);
                        resolve
                            .identifiers
                            .insert(identifier, GTIdentifier::new(span, alias));
                    });
                }
            }
        }

        let module =
            TSConvertModule::convert(&module.module, resolve, config.common.dependencies.clone()).0;

        Ok(Self { path, module })
    }

    fn dependencies(&self) -> Vec<Self::Dependency> {
        vec![]
    }
}

impl Hash for TsProjectModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
