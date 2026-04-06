use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsReference {
    #[visit]
    pub identifier: TsIdentifier,
    pub rel: TsReferenceRel,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum TsReferenceRel {
    Regular,
    Forward,
    SelfRecursive,
}

impl TsReference {
    pub fn new(identifier: TsIdentifier, rel: TsReferenceRel) -> Self {
        Self { identifier, rel }
    }
}

impl From<&str> for TsReference {
    fn from(str: &str) -> Self {
        TsReference::new(str.into(), TsReferenceRel::Regular)
    }
}

impl From<TsIdentifier> for TsReference {
    fn from(identifier: TsIdentifier) -> Self {
        TsReference::new(identifier, TsReferenceRel::Regular)
    }
}
