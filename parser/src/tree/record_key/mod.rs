use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GTRecordKey {
    String(GTSpan),
    Number(GTSpan),
    Int8(GTSpan),
    Int16(GTSpan),
    Int32(GTSpan),
    Int64(GTSpan),
    Int128(GTSpan),
    IntSize(GTSpan),
    IntU8(GTSpan),
    IntU16(GTSpan),
    IntU32(GTSpan),
    IntU64(GTSpan),
    IntU128(GTSpan),
    IntUSize(GTSpan),
    Float32(GTSpan),
    Float64(GTSpan),
}
