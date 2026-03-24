use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSAlias {
    #[visit]
    pub doc: Option<TSDoc>,
    #[visit]
    pub name: TSIdentifier,
    #[visit]
    pub descriptor: TSDescriptor,
}
