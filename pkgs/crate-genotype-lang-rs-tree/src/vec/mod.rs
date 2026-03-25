use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsVec {
    #[visit]
    pub descriptor: RsDescriptor,
}
