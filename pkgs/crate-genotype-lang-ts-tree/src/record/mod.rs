use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsRecord {
    #[visit]
    pub key: TsRecordKey,
    #[visit]
    pub descriptor: TsDescriptor,
}
