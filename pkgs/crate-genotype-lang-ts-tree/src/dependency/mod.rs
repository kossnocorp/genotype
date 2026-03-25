use crate::prelude::internal::*;

pub struct TsDependency {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TsDependencyIdent {
    Local(TsPath),
}

impl TsDependencyIdent {
    pub fn as_path(&self) -> TsPath {
        match self {
            Self::Local(path) => path.clone(),
        }
    }
}

impl GtlDependencyIdent for TsDependencyIdent {}
