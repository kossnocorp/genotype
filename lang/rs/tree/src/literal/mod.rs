mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSLiteral {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}
