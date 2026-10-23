use crate::prelude::internal::*;

impl GtParse for GtLiteralValue {
    fn parse(token: GtToken<'_>, _context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = token.span;
        let token = token
            .into_semantic_children()
            .next()
            .ok_or(GtParseError::UnexpectedEnd(
                span,
                GtNode::Literal,
                "literal value",
            ))?;

        match token.kind {
            GtTokenKind::LiteralString => {
                let token =
                    token
                        .into_semantic_children()
                        .next()
                        .ok_or(GtParseError::UnexpectedEnd(
                            span,
                            GtNode::Literal,
                            "literal string value",
                        ))?;
                Ok(GtLiteralValue::String(token.as_str().into()))
            }

            GtTokenKind::LiteralInteger => {
                let value = token.as_str().replace("_", "").parse().map_err(|_| {
                    GtParseError::Internal(span, GtNode::Literal, "failed to parse integer")
                })?;
                Ok(GtLiteralValue::Integer(value))
            }

            GtTokenKind::LiteralFloat => {
                let value = token.as_str().replace("_", "").parse().map_err(|_| {
                    GtParseError::Internal(span, GtNode::Literal, "failed to parse float")
                })?;
                Ok(GtLiteralValue::Float(value))
            }

            GtTokenKind::LiteralBoolean => {
                let value = token.as_str().parse().map_err(|_| {
                    GtParseError::Internal(span, GtNode::Literal, "failed to parse boolean")
                })?;
                Ok(GtLiteralValue::Boolean(value))
            }

            GtTokenKind::LiteralNull => Ok(GtLiteralValue::Null),

            kind => Err(GtParseError::UnexpectedToken(
                span,
                GtNode::Literal,
                kind,
                "expected literal value",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut tokens = GtParser::parse(GtTokenKind::Literal, "420").unwrap();
        let mut context = GtContext::new("module".into());
        assert_eq!(
            GtLiteralValue::Integer(420),
            GtLiteralValue::parse(tokens.next().unwrap(), &mut context).unwrap()
        );
    }

    #[test]
    fn test_error() {
        let mut tokens = GtParser::parse(GtTokenKind::Object, "{}").unwrap();
        let mut context = GtContext::new("module".into());
        assert_eq!(
            GtParseError::UnexpectedEnd((0, 2).into(), GtNode::Literal, "literal value"),
            GtLiteralValue::parse(tokens.next().unwrap(), &mut context).unwrap_err(),
        );
    }
}
