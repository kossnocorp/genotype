use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone)]
pub struct TSProjectModule {
    pub path: PathBuf,
    pub module: TSModule,
}

impl<'a> GtlProjectModule<'a, TsConfig> for TSProjectModule {
    type Dependency = TSDependencyIdent;

    fn generate(
        config: &'a GtConfigPkg<'a, TsConfig>,
        module: &GTProjectModule,
    ) -> Result<Self> {
        let path = project.config.ts.src_path().join(
            module
                .path
                .as_path()
                .strip_prefix(project.root.as_path())
                .map_err(|_| TSProjectError::BuildModulePath(module.path.as_name()))?
                .with_extension("ts"),
        );

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

        let module = TSConvertModule::convert(
            &module.module,
            resolve,
            config.target.common.dependencies.clone(),
        )
        .0;

        Ok(Self { path, module })
    }

    fn dependencies(&self) -> Vec<Self::Dependency> {
        vec![]
    }
}

impl Hash for TSProjectModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
