use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTRecord {
    pub span: GTSpan,
    pub doc: Option<GTDoc>,
    pub attributes: Vec<GTAttribute>,
    pub key: GTRecordKey,
    pub descriptor: GTDescriptor,
}
