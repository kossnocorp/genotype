use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RSUse {
    #[visit]
    pub dependency: RSDependencyIdent,
    #[visit]
    pub reference: RSUseReference,
}

impl RSUse {
    pub fn new(dependency: RSDependencyIdent, reference: RSUseReference) -> Self {
        Self {
            dependency,
            reference,
        }
    }
}

impl GtlImport for RSUse {}
