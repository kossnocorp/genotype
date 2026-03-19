use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GTLiteralValue {
    Null,
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl GTLiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            GTLiteralValue::Null => "null".to_string(),
            GTLiteralValue::String(value) => value.clone(),
            GTLiteralValue::Integer(value) => value.to_string(),
            GTLiteralValue::Float(value) => value.to_string(),
            GTLiteralValue::Boolean(value) => value.to_string(),
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
