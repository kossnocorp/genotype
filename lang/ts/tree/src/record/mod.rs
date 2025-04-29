use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSRecord {
    pub key: TSRecordKey,
    pub descriptor: TSDescriptor,
}
