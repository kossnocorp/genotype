use crate::prelude::internal::*;
use genotype_lang_py_config::PyPackageManager;
use toml_edit::*;

pub struct PyManifest<'project, 'config> {
    config: &'config GtlConfig<'project, PyConfig>,
}

impl<'project, 'config> GtlManifest<'project, 'config> for PyManifest<'project, 'config> {
    type ProjectModule = PyProjectModule;

    fn new(
        config: &'config GtlConfig<'project, GtlProjectModuleTypeLangConfig<Self::ProjectModule>>,
    ) -> Self
    where
        Self: Sized,
    {
        PyManifest { config }
    }

    fn config(&self) -> &GtlConfig<'_, GtlProjectModuleTypeLangConfig<Self::ProjectModule>> {
        self.config
    }

    fn file_name(&self) -> &'static str {
        "pyproject.toml"
    }

    fn dependencies_key(&self) -> &'static str {
        match self.config().lang_config.lang.manager {
            PyPackageManager::Poetry => "tool.poetry.dependencies",
            // TODO: Currently, it is unused because uv stores dependencies as an array rather
            // than a table. We may want to update the base logic to better reflect this difference.
            PyPackageManager::Uv => "project.dependencies",
        }
    }

    fn base(&self) -> String {
        let module = self.config().lang_config.module.as_str();
        let python_version = self.config().lang_config.lang.version.version_str();

        match self.config().lang_config.lang.manager {
            PyPackageManager::Poetry => {
                let mut source = format!(
                    r#"[tool.poetry]
packages = [{{ include = "{module}" }}]
"#
                );

                if let Some(version) = self.config().project_version {
                    source.push_str(format!("version = \"{version}\"\n").as_str());
                }

                source.push_str(
                    format!(
                        r#"
[tool.poetry.dependencies]
python = "{python_version}"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
"#
                    )
                    .as_str(),
                );

                source
            }
            PyPackageManager::Uv => {
                let mut source = format!(
                    r#"[project]
requires-python = "{python_version}"
"#,
                    python_version = poetry_req_to_uv_req(python_version)
                );

                if let Some(version) = self.config().project_version {
                    source.push_str(format!("version = \"{version}\"\n").as_str());
                }

                source.push_str(
                    format!(
                        r#"
[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.hatch.build.targets.wheel]
packages = ["{module}"]
"#
                    )
                    .as_str(),
                );

                source
            }
        }
    }

    fn insert_dependencies(
        &self,
        manifest: &mut DocumentMut,
        deps: &IndexSet<PyDependencyIdent>,
    ) -> Result<(), Box<dyn GtlError>> {
        match self.config().lang_config.lang.manager {
            PyPackageManager::Poetry => {
                let manifest_deps = manifest
                    .drill_table_mut(self.dependencies_key())
                    .map_err(|err| GtlManifestError::edit(err))?;

                for dep in deps.iter() {
                    if let Some((key, value)) = Self::dependency_as_kv(dep) {
                        manifest_deps[&key] = value.into();
                    }
                }

                if manifest_deps.is_empty() {
                    manifest.remove(self.dependencies_key());
                }

                Ok(())
            }

            PyPackageManager::Uv => {
                let project = manifest
                    .drill_table_mut("project")
                    .map_err(|err| GtlManifestError::edit(err))?;

                let mut project_deps = deps
                    .iter()
                    .filter_map(|dep| {
                        Self::dependency_as_kv(dep).map(|(name, version)| {
                            let version = version
                                .as_str()
                                .map(poetry_req_to_uv_req)
                                .unwrap_or_else(|| version.to_string());
                            format!("{name}{version}")
                        })
                    })
                    .collect::<Vec<_>>();
                project_deps.sort();

                if project_deps.is_empty() {
                    project.remove("dependencies");
                } else {
                    let mut dependencies = Array::new();
                    for dep in project_deps.into_iter() {
                        dependencies.push(dep.as_str());
                    }
                    project["dependencies"] = Item::Value(Value::Array(dependencies));
                }

                Ok(())
            }
        }
    }

    fn dependency_as_kv(ident: &PyDependencyIdent) -> Option<(String, Value)> {
        match ident {
            PyDependencyIdent::Runtime => Some(("genotype-runtime".into(), "^0.4".into())),
            PyDependencyIdent::TypingExtensions => Some(("typing-extensions".into(), "^4".into())),
            PyDependencyIdent::Pydantic => Some(("pydantic".into(), "^2.9".into())),
            _ => None,
        }
    }
}

fn poetry_req_to_uv_req(req: &str) -> String {
    let Some(version) = req.strip_prefix('^') else {
        return req.to_string();
    };

    let numbers = version
        .split('.')
        .map(|value| value.parse::<u64>())
        .collect::<std::result::Result<Vec<_>, _>>();

    let Ok(numbers) = numbers else {
        return req.to_string();
    };

    if numbers.is_empty() {
        return req.to_string();
    }

    let upper = if numbers[0] > 0 {
        format!("{}", numbers[0] + 1)
    } else if numbers.len() == 1 {
        "1".into()
    } else if numbers.len() == 2 || numbers[1] > 0 {
        format!("0.{}", numbers[1] + 1)
    } else {
        format!("0.0.{}", numbers[2] + 1)
    };

    format!(">={version},<{upper}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poetry_req_to_uv_req() {
        assert_eq!(poetry_req_to_uv_req("^3.8"), ">=3.8,<4");
        assert_eq!(poetry_req_to_uv_req("^2.9"), ">=2.9,<3");
        assert_eq!(poetry_req_to_uv_req("^0.4"), ">=0.4,<0.5");
        assert_eq!(poetry_req_to_uv_req("^0.0.4"), ">=0.0.4,<0.0.5");
    }
}
