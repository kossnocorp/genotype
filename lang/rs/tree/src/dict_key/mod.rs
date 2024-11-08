mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSDictKey {
    Int,
    Float,
    String,
    Boolean,
}
