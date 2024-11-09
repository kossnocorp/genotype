mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSPrimitive {
    Unit,
    Boolean,
    String,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Int,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    UInt,
    Float32,
    Float64,
}
