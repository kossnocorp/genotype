use crate::prelude::internal::*;

pub struct PYDependency {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PYDependencyIdent {
    Path(PYPath),
    Runtime,
    Typing,
    TypingExtensions,
    Pydantic,
}

impl PYDependencyIdent {
    pub fn as_path(&self) -> PYPath {
        match self {
            Self::Path(path) => path.clone(),
            Self::Runtime => "genotype".into(),
            Self::Typing => "typing".into(),
            Self::TypingExtensions => "typing_extensions".into(),
            Self::Pydantic => "pydantic".into(),
        }
    }

    pub fn external(&self) -> Option<PYDependencyExternal> {
        match self {
            Self::Runtime => Some(PYDependencyExternal {
                name: "genotype-runtime".into(),
                version: "^0.4".into(),
            }),
            Self::TypingExtensions => Some(PYDependencyExternal {
                name: "typing-extensions".into(),
                version: "^4".into(),
            }),
            Self::Pydantic => Some(PYDependencyExternal {
                name: "pydantic".into(),
                version: "^2.9".into(),
            }),
            _ => None,
        }
    }
}

impl GtlDependencyIdent for PYDependencyIdent {}

impl From<&str> for PYDependencyIdent {
    fn from(str: &str) -> Self {
        PYDependencyIdent::Path(str.into())
    }
}

pub struct PYDependencyExternal {
    pub name: String,
    pub version: String,
}
