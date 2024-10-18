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
}
