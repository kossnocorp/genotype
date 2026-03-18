use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct RSVec {
    pub descriptor: RSDescriptor,
}
