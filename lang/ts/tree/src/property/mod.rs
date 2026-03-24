use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSProperty {
    #[visit]
    pub doc: Option<TSDoc>,
    #[visit]
    pub name: TSKey,
    #[visit]
    pub descriptor: TSDescriptor,
    pub required: bool,
}
