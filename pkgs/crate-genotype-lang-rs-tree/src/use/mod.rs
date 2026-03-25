use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsUse {
    #[visit]
    pub dependency: RsDependencyIdent,
    #[visit]
    pub reference: RsUseReference,
}

impl RsUse {
    pub fn new(dependency: RsDependencyIdent, reference: RsUseReference) -> Self {
        Self {
            dependency,
            reference,
        }
    }
}

impl GtlImport for RsUse {}
