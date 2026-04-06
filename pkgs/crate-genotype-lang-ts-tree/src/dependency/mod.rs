use crate::prelude::internal::*;

pub struct TsDependency {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Visitor)]
pub enum TsDependencyIdent {
    Local(#[visit] TsPath),
    Zod,
}

impl TsDependencyIdent {
    pub fn as_path(&self) -> TsPath {
        match self {
            Self::Local(path) => path.clone(),
            Self::Zod => "zod".into(),
        }
    }
}

impl GtlDependencyIdent for TsDependencyIdent {}
