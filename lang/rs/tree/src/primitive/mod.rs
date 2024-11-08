mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSPrimitive {
    Boolean,
    String,
    Int,
    Float,
    None,
}
