use crate::prelude::internal::*;
use toml_edit::*;

impl<'a> GtlProjectManifest<'a> for PyProject<'a> {
    const FILE_NAME: &'static str = "pyproject.toml";
    const MANIFEST_DEPENDENCIES_KEY: &'static str = "tool.poetry.dependencies";

    type Dependency = PyProjectManifestDependency;
    type LangConfig = PyConfig;

    fn config(&'a self) -> &'a GtConfigPkg<'a, Self::LangConfig> {
        &self.config
    }

    fn base_manifest(&self) -> String {
        let module = self.config.target.module.as_str();
        let python_version = self.config.target.lang.version.version_str();

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
