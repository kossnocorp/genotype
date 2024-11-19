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
    pub fn to_span(&self) -> GTSpan {
        match self {
            GTLiteral::String(span, _) => span,
            GTLiteral::Integer(span, _) => span,
            GTLiteral::Float(span, _) => span,
            GTLiteral::Boolean(span, _) => span,
        }
        .clone()
    }

    pub fn to_string(&self) -> String {
        match self {
            GTLiteral::String(_, value) => value.clone(),
            GTLiteral::Integer(_, value) => value.to_string(),
            GTLiteral::Float(_, value) => value.to_string(),
            GTLiteral::Boolean(_, value) => value.to_string(),
        }
    }

    pub fn render_float(value: &f64) -> String {
        if value.fract() == 0.0 {
            format!("{:.1}", value)
        } else {
            value.to_string()
        }
    }

    pub fn render_string(value: &String) -> String {
        format!("\"{}\"", value.escape_default())
    }
}
