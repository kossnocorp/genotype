use crate::prelude::internal::*;

pub struct TSDependency {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TSDependencyIdent {
    Local(TSPath),
}

impl TSDependencyIdent {
    pub fn as_path(&self) -> TSPath {
        match self {
            Self::Local(path) => path.clone(),
        }
    }
}

impl GtlDependencyIdent for TSDependencyIdent {}
