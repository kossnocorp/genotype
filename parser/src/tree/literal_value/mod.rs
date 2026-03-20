use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Clone, Serialize)]
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

impl PartialEq for GTLiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GTLiteralValue::Null, GTLiteralValue::Null) => true,

            (GTLiteralValue::String(a), GTLiteralValue::String(b)) => a == b,

            (GTLiteralValue::Integer(a), GTLiteralValue::Integer(b)) => a == b,

            (GTLiteralValue::Float(a), GTLiteralValue::Float(b)) => {
                // Normalize -0.0 to 0.0
                let a = if a == &-0.0 { 0.0 } else { *a };
                let b = if b == &-0.0 { 0.0 } else { *b };

                // Treat NaN as equal to NaN
                if a.is_nan() && b.is_nan() {
                    true
                } else {
                    a == b
                }
            }

            (GTLiteralValue::Boolean(a), GTLiteralValue::Boolean(b)) => a == b,

            _ => false,
        }
    }
}

impl Eq for GTLiteralValue {}

impl Hash for GTLiteralValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            GTLiteralValue::Null => state.write_u8(0),

            GTLiteralValue::String(value) => {
                value.hash(state);
            }

            GTLiteralValue::Integer(value) => {
                value.hash(state);
            }

            GTLiteralValue::Float(value) => {
                state.write_u8(3);
                let mut bits = value.to_bits();

                // Treat all NaN values the same
                if value.is_nan() {
                    bits = f64::NAN.to_bits();
                } else if bits == (-0.0f64).to_bits() {
                    // Normalize -0.0 to 0.0
                    bits = 0.0f64.to_bits();
                }

                bits.hash(state);
            }

            GTLiteralValue::Boolean(value) => {
                value.hash(state);
            }
        }
    }
}
