use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct TSProperty {
    pub doc: Option<TSDoc>,
    pub name: TSKey,
    pub descriptor: TSDescriptor,
    pub required: bool,
}
