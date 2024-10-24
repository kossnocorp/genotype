use crate::GTSpan;

use super::{descriptor::GTDescriptor, GTRecordKey};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTRecord {
    pub span: GTSpan,
    pub key: GTRecordKey,
    pub descriptor: GTDescriptor,
}
