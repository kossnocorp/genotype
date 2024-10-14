use crate::GTSpan;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub enum GTLiteral {
    String(GTSpan, String),
    Integer(GTSpan, i64),
    Float(GTSpan, f64),
    Boolean(GTSpan, bool),
}
