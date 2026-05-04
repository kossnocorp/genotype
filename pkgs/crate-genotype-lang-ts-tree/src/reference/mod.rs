use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsReference {
    #[visit]
    pub identifier: TsIdentifier,
    #[visit]
    pub arguments: Vec<TsDescriptor>,
    pub rel: TsReferenceRel,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum TsReferenceRel {
    Regular,
    Forward,
    SelfRecursive,
}

impl TsReference {
    pub fn new(
        identifier: TsIdentifier,
        arguments: Vec<TsDescriptor>,
        rel: TsReferenceRel,
    ) -> Self {
        Self {
            identifier,
            arguments,
            rel,
        }
    }
}

impl From<&str> for TsReference {
    fn from(str: &str) -> Self {
        TsReference::new(str.into(), vec![], TsReferenceRel::Regular)
    }
}

impl From<TsIdentifier> for TsReference {
    fn from(identifier: TsIdentifier) -> Self {
        TsReference::new(identifier, vec![], TsReferenceRel::Regular)
    }
}
