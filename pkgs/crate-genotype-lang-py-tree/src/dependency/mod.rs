use crate::prelude::internal::*;

pub struct PyDependency {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Visitor)]
pub enum PyDependencyIdent {
    Path(#[visit] PyPath),
    Runtime,
    Typing,
    TypingExtensions,
    Pydantic,
}

impl PyDependencyIdent {
    pub fn as_path(&self) -> PyPath {
        match self {
            Self::Path(path) => path.clone(),
            Self::Runtime => "genotype".into(),
            Self::Typing => "typing".into(),
            Self::TypingExtensions => "typing_extensions".into(),
            Self::Pydantic => "pydantic".into(),
        }
    }

    pub fn external(&self) -> Option<PyDependencyExternal> {
        match self {
            Self::Runtime => Some(PyDependencyExternal {
                name: "genotype-runtime".into(),
                version: "^0.4".into(),
            }),
            Self::TypingExtensions => Some(PyDependencyExternal {
                name: "typing-extensions".into(),
                version: "^4".into(),
            }),
            Self::Pydantic => Some(PyDependencyExternal {
                name: "pydantic".into(),
                version: "^2.9".into(),
            }),
            _ => None,
        }
    }
}

impl GtlDependencyIdent for PyDependencyIdent {}

impl From<&str> for PyDependencyIdent {
    fn from(str: &str) -> Self {
        PyDependencyIdent::Path(str.into())
    }
}

pub struct PyDependencyExternal {
    pub name: String,
    pub version: String,
}
