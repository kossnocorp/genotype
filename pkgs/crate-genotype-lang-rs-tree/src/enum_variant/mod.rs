use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsEnumVariant {
    #[visit]
    pub doc: Option<RsDoc>,
    #[visit]
    pub attributes: Vec<RsAttribute>,
    #[visit]
    pub name: RsIdentifier,
    #[visit]
    pub descriptor: Option<RsEnumVariantDescriptor>,
}
