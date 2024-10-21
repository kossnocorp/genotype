use crate::PYPath;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PYDependency {
    Local(PYPath),
    Runtime,
    Typing,
    Pydantic,
}

impl PYDependency {
    pub fn as_path(&self) -> PYPath {
        match self {
            Self::Local(path) => path.clone(),
            Self::Runtime => "genotype".into(),
            Self::Typing => "typing".into(),
            Self::Pydantic => "pydantic".into(),
        }
    }

    pub fn external_str(&self) -> Option<String> {
        match self {
            Self::Runtime => Some(r#"genotype = "^0.3""#.into()),
            Self::Pydantic => Some(r#"pydantic = "^2""#.into()),
            _ => None,
        }
    }
}
