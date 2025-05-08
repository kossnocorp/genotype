use crate::prelude::internal::*;
use toml_edit::*;

impl<'a> GtlProjectManifest<'a> for PyProject<'a> {
    const FILE_NAME: &'static str = "pyproject.toml";

    type Dependency = PyProjectManifestDependency;
    type LangConfig = PyConfig;

    fn config(&'a self) -> &'a GtConfigPkg<'a, Self::LangConfig> {
        &self.config
    }
}

pub struct PyProjectManifestDependency {}

impl GtlProjectManifestDependency for PyProjectManifestDependency {
    type DependencyIdent = PYDependencyIdent;

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
