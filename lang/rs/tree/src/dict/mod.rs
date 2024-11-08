use crate::{descriptor::RSDescriptor, RSDictKey};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSDict {
    pub key: RSDictKey,
    pub descriptor: RSDescriptor,
}
