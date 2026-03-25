use crate::prelude::internal::*;

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

impl GTLiteralValue {
    pub fn parse(pair: Pair<'_, Rule>, _context: &mut GTContext) -> Result<Self, GTParseError> {
        let span: GTSpan = pair.as_span().into();
        let else_err = || GTParseError::Internal(span.clone(), GTNode::Literal);
        let pair = pair.into_inner().next().ok_or_else(else_err)?;

        match pair.as_rule() {
            Rule::literal_string => {
                let pair = pair.into_inner().next().ok_or_else(else_err)?;
                Ok(GTLiteralValue::String(pair.as_str().into()))
            }

            Rule::literal_integer => {
                let value = pair
                    .as_str()
                    .replace("_", "")
                    .parse()
                    .map_err(|_| else_err())?;
                Ok(GTLiteralValue::Integer(value))
            }

            Rule::literal_float => {
                let value = pair
                    .as_str()
                    .replace("_", "")
                    .parse()
                    .map_err(|_| else_err())?;
                Ok(GTLiteralValue::Float(value))
            }

            Rule::literal_boolean => {
                let value = pair.as_str().parse().map_err(|_| else_err())?;
                Ok(GTLiteralValue::Boolean(value))
            }

            Rule::literal_null => Ok(GTLiteralValue::Null),

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
        let mut context = GTContext::new("module".into());
        assert_eq!(
            GTLiteralValue::Integer(420),
            GTLiteralValue::parse(pairs.next().unwrap(), &mut context).unwrap()
        );
    }

    #[test]
    fn test_error() {
        let mut pairs = GenotypeParser::parse(Rule::object, "{}").unwrap();
        let mut context = GTContext::new("module".into());
        assert_eq!(
            GTParseError::Internal((0, 2).into(), GTNode::Literal),
            GTLiteralValue::parse(pairs.next().unwrap(), &mut context).unwrap_err(),
        );
    }
}
