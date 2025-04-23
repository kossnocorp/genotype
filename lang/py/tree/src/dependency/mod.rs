use crate::prelude::internal::*;

pub struct PYDependency {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PYDependencyIdent {
    Local(PYPath),
    Runtime,
    Typing,
    TypingExtensions,
    Pydantic,
}

impl PYDependencyIdent {
    pub fn as_path(&self) -> PYPath {
        match self {
            Self::Local(path) => path.clone(),
            Self::Runtime => "genotype".into(),
            Self::Typing => "typing".into(),
            Self::TypingExtensions => "typing_extensions".into(),
            Self::Pydantic => "pydantic".into(),
        }
    }

    pub fn external_str(&self) -> Option<String> {
        match self {
            Self::Runtime => Some(r#"genotype-runtime = "^0.4""#.into()),
            Self::TypingExtensions => Some(r#"typing-extensions = "^4""#.into()),
            Self::Pydantic => Some(r#"pydantic = "^2.9""#.into()),
            _ => None,
        }
    }
}

impl GtlDependencyIdent for PYDependencyIdent {}
