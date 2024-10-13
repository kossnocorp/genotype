mod parse;

#[derive(Debug, PartialEq, Clone)]
pub enum GTLiteral {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}
