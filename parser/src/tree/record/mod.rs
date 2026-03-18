use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTRecord {
    pub span: GTSpan,
    pub key: GTRecordKey,
    pub descriptor: GTDescriptor,
}
