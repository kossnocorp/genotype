use crate::prelude::internal::*;
use toml_edit::*;

pub struct PyProjectManifest {}

impl GtlProjectManifest for PyProjectManifest {
    const FILE_NAME: &'static str = "pyproject.toml";

    type ManifestDependency = PyProjectManifestDependency;
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
