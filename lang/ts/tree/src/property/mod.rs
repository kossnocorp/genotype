use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSProperty {
    pub doc: Option<TSDoc>,
    pub name: TSKey,
    pub descriptor: TSDescriptor,
    pub required: bool,
}
