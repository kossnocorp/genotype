use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsMap {
    #[visit]
    pub key: RsDescriptor,
    #[visit]
    pub descriptor: RsDescriptor,
}
