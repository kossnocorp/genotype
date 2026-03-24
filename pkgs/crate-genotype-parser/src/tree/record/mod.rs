use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GTRecord {
    pub span: GTSpan,
    #[visit]
    pub doc: Option<GTDoc>,
    #[visit]
    pub attributes: Vec<GTAttribute>,
    #[visit]
    pub key: GTRecordKey,
    #[visit]
    pub descriptor: GTDescriptor,
}
