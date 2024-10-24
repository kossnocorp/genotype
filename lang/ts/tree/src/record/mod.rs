use crate::{descriptor::TSDescriptor, TSRecordKey};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSRecord {
    pub key: TSRecordKey,
    pub descriptor: TSDescriptor,
}
