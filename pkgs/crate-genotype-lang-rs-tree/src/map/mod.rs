use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RSMap {
    #[visit]
    pub key: RSDescriptor,
    #[visit]
    pub descriptor: RSDescriptor,
}
