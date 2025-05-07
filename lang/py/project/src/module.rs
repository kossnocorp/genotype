use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone)]
pub struct PyProjectModule {
    pub name: String,
    pub path: PathBuf,
    pub module: PYModule,
}

impl GtlProjectModule for PyProjectModule {
    type Dependency = PYDependencyIdent;

    fn generate(project: &GtProject, module: &GTProjectModule) -> Result<Self> {
        let relative_path = module
            .path
            .as_path()
            .strip_prefix(project.root.as_path())
            .map_err(|_| PyProjectError::BuildModulePath(module.path.as_name()))?;
        let name = py_parse_module_path(
            relative_path
                .with_extension("")
                .as_os_str()
                .to_str()
                .unwrap()
                .to_string(),
        );
        let path = project
            .config
            .py
            .src_path()
            .join(relative_path.with_extension("py"));

        let mut resolve = PYConvertResolve::default();
        let mut prefixes: HashMap<String, u8> = HashMap::new();

        // [TODO] I'm pretty sure I can extract it and share with TypeScript
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

                        resolve.globs.insert(import.path.clone(), prefix.clone());

                        references.iter().for_each(|(reference, _)| {
                            let identifier = (*reference).clone();
                            let span = identifier.0.clone();
                            let alias_str = format!("{}.{}", prefix, identifier.1);
                            let alias = GTIdentifier::new(span, alias_str);
                            resolve.identifiers.insert(identifier.clone(), alias);
                            resolve.imported.insert(identifier);
                        });
                    }
                }

                GTImportReference::Name(_, identifier) => {
                    resolve.imported.insert(identifier.clone());
                }

                GTImportReference::Names(_, identifiers) => {
                    identifiers.iter().for_each(|name| {
                        resolve.imported.insert(
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

        let module = PYConvertModule::convert(
            &module.module,
            &resolve,
            &project.config.py.lang,
            project.config.py.common.dependencies.clone(),
        )
        .0;

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
