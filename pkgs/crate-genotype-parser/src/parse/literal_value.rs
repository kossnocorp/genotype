use crate::prelude::internal::*;

impl GtParse for GtLiteralValue {
    fn parse(pair: Pair<'_, Rule>, _context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = (pair.as_span().start(), pair.as_span().end()).into();
        let pair = pair.into_inner().next().ok_or(GtParseError::UnexpectedEnd(
            span,
            GtNode::Literal,
            "literal value",
        ))?;

        match pair.as_rule() {
            Rule::literal_string => {
                let pair = pair.into_inner().next().ok_or(GtParseError::UnexpectedEnd(
                    span,
                    GtNode::Literal,
                    "literal string value",
                ))?;
                Ok(GtLiteralValue::String(pair.as_str().into()))
            }

            Rule::literal_integer => {
                let value = pair.as_str().replace("_", "").parse().map_err(|_| {
                    GtParseError::Internal(span, GtNode::Literal, "failed to parse integer")
                })?;
                Ok(GtLiteralValue::Integer(value))
            }

            Rule::literal_float => {
                let value = pair.as_str().replace("_", "").parse().map_err(|_| {
                    GtParseError::Internal(span, GtNode::Literal, "failed to parse float")
                })?;
                Ok(GtLiteralValue::Float(value))
            }

            Rule::literal_boolean => {
                let value = pair.as_str().parse().map_err(|_| {
                    GtParseError::Internal(span, GtNode::Literal, "failed to parse boolean")
                })?;
                Ok(GtLiteralValue::Boolean(value))
            }

            Rule::literal_null => Ok(GtLiteralValue::Null),

            rule => Err(GtParseError::UnexpectedRule(
                span,
                GtNode::Literal,
                rule,
                "expected literal value",
            )),
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
            GtParseError::UnexpectedEnd((0, 2).into(), GtNode::Literal, "literal value"),
            GtLiteralValue::parse(pairs.next().unwrap(), &mut context).unwrap_err(),
        );
    }
}
