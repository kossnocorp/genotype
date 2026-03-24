use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSRecord {
    #[visit]
    pub key: TSRecordKey,
    #[visit]
    pub descriptor: TSDescriptor,
}
