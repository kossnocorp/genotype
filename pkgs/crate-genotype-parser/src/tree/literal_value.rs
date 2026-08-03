use crate::prelude::internal::*;

#[derive(Debug, Clone, Serialize)]
pub enum GtLiteralValue {
    Null,
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl GtLiteralValue {
    pub fn render_float(value: &f64) -> String {
        if value.fract() == 0.0 {
            format!("{value:.1}")
        } else {
            value.to_string()
        }
    }

    pub fn render_string(value: &str) -> String {
        format!("\"{}\"", value.escape_default())
    }
}

impl Display for GtLiteralValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(formatter, "null"),
            Self::String(value) => write!(formatter, "{}", Self::render_string(value)),
            Self::Integer(value) => write!(formatter, "{value}"),
            Self::Float(value) => write!(formatter, "{}", Self::render_float(value)),
            Self::Boolean(value) => write!(formatter, "{value}"),
        }
    }
}

impl PartialEq for GtLiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => (a.is_nan() && b.is_nan()) || a == b,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            _ => false,
        }
    }
}
impl Eq for GtLiteralValue {}
impl Hash for GtLiteralValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Null => {}
            Self::String(v) => v.hash(state),
            Self::Integer(v) => v.hash(state),
            Self::Float(v) => {
                let bits = if v.is_nan() {
                    f64::NAN.to_bits()
                } else if *v == 0.0 {
                    0
                } else {
                    v.to_bits()
                };
                bits.hash(state)
            }
            Self::Boolean(v) => v.hash(state),
        }
    }
}
