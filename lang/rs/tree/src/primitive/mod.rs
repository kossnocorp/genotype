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
    IntSize,
    IntU8,
    IntU16,
    IntU32,
    IntU64,
    IntU128,
    IntUSize,
    Float32,
    Float64,
}
