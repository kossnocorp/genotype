mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSLiteral {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}
