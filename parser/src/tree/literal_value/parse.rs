use crate::prelude::internal::*;

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
