mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSLiteral {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}
