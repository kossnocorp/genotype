use crate::prelude::internal::*;

pub struct TsDependency {}

impl GtlDependency for TsDependency {
    type Ident = TsDependencyIdent;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Visitor)]
pub enum TsDependencyIdent {
    Local(#[visit] TsPath),
    Zod,
}

impl TsDependencyIdent {}

impl GtlDependencyIdent for TsDependencyIdent {
    type Path = TsPath;

    fn as_path(&self) -> Self::Path {
        match self {
            Self::Local(path) => path.clone(),
            Self::Zod => "zod".into(),
        }
    }
}
