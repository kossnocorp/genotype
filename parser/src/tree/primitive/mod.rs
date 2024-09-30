mod parse;

#[derive(Debug, PartialEq, Clone)]
pub enum GTPrimitive {
    Boolean,
    String,
    Int,
    Float,
}
