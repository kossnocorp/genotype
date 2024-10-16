mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYLiteral {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}
