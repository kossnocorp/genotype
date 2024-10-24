mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYDictKey {
    Int,
    Float,
    String,
    Boolean,
}
