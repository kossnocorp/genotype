use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub enum GtRecordKey {
    Reference(#[visit] GtReference),
    String(GtSpan),
    Number(GtSpan),
    Int8(GtSpan),
    Int16(GtSpan),
    Int32(GtSpan),
    Int64(GtSpan),
    Int128(GtSpan),
    IntSize(GtSpan),
    IntU8(GtSpan),
    IntU16(GtSpan),
    IntU32(GtSpan),
    IntU64(GtSpan),
    IntU128(GtSpan),
    IntUSize(GtSpan),
    Float32(GtSpan),
    Float64(GtSpan),
}
