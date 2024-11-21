use crate::{TSDoc, TSIdentifier, TSPrimitive};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSBranded {
    pub doc: Option<TSDoc>,
    pub name: TSIdentifier,
    pub primitive: TSPrimitive,
}
