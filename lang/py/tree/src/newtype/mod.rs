use crate::{identifier::PYIdentifier, PYDoc, PYPrimitive};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYNewtype {
    pub doc: Option<PYDoc>,
    pub name: PYIdentifier,
    pub primitive: PYPrimitive,
}
