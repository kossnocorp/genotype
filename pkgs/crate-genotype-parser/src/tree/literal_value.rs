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
            format!("{:.1}", value)
        } else {
            value.to_string()
        }
    }

    pub fn render_string(value: &str) -> String {
        format!("\"{}\"", value.escape_default())
    }
}

impl Display for GtLiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GtLiteralValue::Null => write!(f, "null"),
            GtLiteralValue::String(value) => write!(f, "{}", Self::render_string(value)),
            GtLiteralValue::Integer(value) => write!(f, "{}", value),
            GtLiteralValue::Float(value) => write!(f, "{}", Self::render_float(value)),
            GtLiteralValue::Boolean(value) => write!(f, "{}", value),
        }
    }
}

impl PartialEq for GtLiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GtLiteralValue::Null, GtLiteralValue::Null) => true,

            (GtLiteralValue::String(a), GtLiteralValue::String(b)) => a == b,

            (GtLiteralValue::Integer(a), GtLiteralValue::Integer(b)) => a == b,

            (GtLiteralValue::Float(a), GtLiteralValue::Float(b)) => {
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

            (GtLiteralValue::Boolean(a), GtLiteralValue::Boolean(b)) => a == b,

            _ => false,
        }
    }
}

impl Eq for GtLiteralValue {}

impl Hash for GtLiteralValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            GtLiteralValue::Null => state.write_u8(0),

            GtLiteralValue::String(value) => {
                value.hash(state);
            }

            GtLiteralValue::Integer(value) => {
                value.hash(state);
            }

            GtLiteralValue::Float(value) => {
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

            GtLiteralValue::Boolean(value) => {
                value.hash(state);
            }
        }
    }
}

impl GtLiteralValue {
    pub fn parse(pair: Pair<'_, Rule>, _context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = pair.as_span().into();
        let else_err = || GtParseError::InternalLegacy(span, GtNode::Literal);
        let pair = pair.into_inner().next().ok_or_else(else_err)?;

        match pair.as_rule() {
            Rule::literal_string => {
                let pair = pair.into_inner().next().ok_or_else(else_err)?;
                Ok(GtLiteralValue::String(pair.as_str().into()))
            }

            Rule::literal_integer => {
                let value = pair
                    .as_str()
                    .replace("_", "")
                    .parse()
                    .map_err(|_| else_err())?;
                Ok(GtLiteralValue::Integer(value))
            }

            Rule::literal_float => {
                let value = pair
                    .as_str()
                    .replace("_", "")
                    .parse()
                    .map_err(|_| else_err())?;
                Ok(GtLiteralValue::Float(value))
            }

            Rule::literal_boolean => {
                let value = pair.as_str().parse().map_err(|_| else_err())?;
                Ok(GtLiteralValue::Boolean(value))
            }

            Rule::literal_null => Ok(GtLiteralValue::Null),

            _ => Err(else_err()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::literal, "420").unwrap();
        let mut context = GtContext::new("module".into());
        assert_eq!(
            GtLiteralValue::Integer(420),
            GtLiteralValue::parse(pairs.next().unwrap(), &mut context).unwrap()
        );
    }

    #[test]
    fn test_error() {
        let mut pairs = GenotypeParser::parse(Rule::object, "{}").unwrap();
        let mut context = GtContext::new("module".into());
        assert_eq!(
            GtParseError::InternalLegacy((0, 2).into(), GtNode::Literal),
            GtLiteralValue::parse(pairs.next().unwrap(), &mut context).unwrap_err(),
        );
    }
}
