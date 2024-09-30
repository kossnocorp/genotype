mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSPrimitive {
    String,
    Number,
    Boolean,
    Null,
    Undefined,
}
