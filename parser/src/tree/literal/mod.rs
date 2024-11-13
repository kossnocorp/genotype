use crate::GTSpan;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub enum GTLiteral {
    String(GTSpan, String),
    Integer(GTSpan, i64),
    Float(GTSpan, f64),
    Boolean(GTSpan, bool),
}

impl GTLiteral {
    pub fn to_string(&self) -> String {
        match self {
            GTLiteral::String(_, value) => value.clone(),
            GTLiteral::Integer(_, value) => value.to_string(),
            GTLiteral::Float(_, value) => value.to_string(),
            GTLiteral::Boolean(_, value) => value.to_string(),
        }
    }
}
