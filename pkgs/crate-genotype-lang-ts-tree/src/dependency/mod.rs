use crate::prelude::internal::*;

pub struct TsDependency {}

impl GtlDependency for TsDependency {
    type Ident = TsDependencyIdent;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Visitor)]
pub enum TsDependencyIdent {
    Local(#[visit] TsPath),
    Zod,
    Effect,
}

impl TsDependencyIdent {
    pub fn as_path(&self) -> TsPath {
        match self {
            Self::Local(path) => path.clone(),
            Self::Effect => "effect".into(),
            Self::Zod => "zod".into(),
        }
    }
}

impl GtlDependencyIdent for TsDependencyIdent {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_effect_dependency_path() {
        assert_eq!(TsDependencyIdent::Effect.as_path(), TsPath::from("effect"));
    }
}
