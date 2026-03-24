use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSBranded {
    #[visit]
    pub doc: Option<TSDoc>,
    #[visit]
    pub name: TSIdentifier,
    #[visit]
    pub primitive: TSPrimitive,
}
