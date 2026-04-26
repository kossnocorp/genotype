use crate::prelude::internal::*;
use genotype_lang_py_config::PyPackageManager;
use toml_edit::*;

impl<'a> GtlProjectManifest<'a> for PyProject<'a> {
    const FILE_NAME: &'static str = "pyproject.toml";
    const MANIFEST_DEPENDENCIES_KEY: &'static str = "tool.poetry.dependencies";

    type Dependency = PyProjectManifestDependency;
    type LangConfig = PyConfig;

    fn config(&'a self) -> &'a GtpPkgConfig<'a, Self::LangConfig> {
        &self.config
    }

    fn base_manifest(&self) -> String {
        let module = self.config.target.module.as_str();
        let python_version = self.config.target.lang.version.version_str();

        match self.config.target.lang.manager {
            PyPackageManager::Poetry => {
                let mut source = format!(
                    r#"[tool.poetry]
packages = [{{ include = "{module}" }}]
"#
                );

                if let Some(version) = self.config.version {
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

                if let Some(version) = self.config.version {
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
        deps: &'a Vec<
            <<Self as GtlProjectManifest<'a>>::Dependency as GtlProjectManifestDependency>::DependencyIdent,
        >,
    ) -> Result<()> {
        match self.config.target.lang.manager {
            PyPackageManager::Poetry => {
                let manifest_deps = manifest["tool"]["poetry"]["dependencies"]
                    .as_table_mut()
                    .ok_or(GtlProjectError::ManifestDepsAccess(Self::FILE_NAME))?;

                for dep in deps.iter() {
                    if let Some((key, value)) = Self::Dependency::as_kv(dep) {
                        manifest_deps[&key] = value.into();
                    }
                }

                if manifest_deps.is_empty() {
                    manifest.remove(Self::MANIFEST_DEPENDENCIES_KEY);
                }

                Ok(())
            }

            PyPackageManager::Uv => {
                let project = manifest["project"]
                    .as_table_mut()
                    .ok_or(GtlProjectError::ManifestDepsAccess(Self::FILE_NAME))?;

                let mut project_deps = deps
                    .iter()
                    .filter_map(|dep| {
                        Self::Dependency::as_kv(dep).map(|(name, version)| {
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

pub struct PyProjectManifestDependency {}

impl GtlProjectManifestDependency for PyProjectManifestDependency {
    type DependencyIdent = PyDependencyIdent;

    fn as_kv(ident: &Self::DependencyIdent) -> Option<(String, Value)> {
        match ident {
            Self::DependencyIdent::Runtime => Some(("genotype-runtime".into(), "^0.4".into())),
            Self::DependencyIdent::TypingExtensions => {
                Some(("typing-extensions".into(), "^4".into()))
            }
            Self::DependencyIdent::Pydantic => Some(("pydantic".into(), "^2.9".into())),
            _ => None,
        }
    }
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
