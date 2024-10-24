use crate::GTSpan;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub enum GTRecordKey {
    String(GTSpan),
    Int(GTSpan),
    Float(GTSpan),
    Boolean(GTSpan),
}
