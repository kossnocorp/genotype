use crate::prelude::internal::*;

impl GtParse for GtAttributeProperty {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = token.span;

        let mut inner = token.into_semantic_children();
        let token = inner.next().ok_or(GtParseError::UnexpectedEnd(
            span,
            GtNode::AttributeProperty,
            "attribute property inner",
        ))?;

        let property = parse(inner, token, ParseState::Name(span), context)?;

        Ok(property)
    }
}

enum ParseState {
    Name(GtSpan),
    Value(GtSpan, GtAttributeKey),
}

fn parse(
    mut inner: GtTokens<'_>,
    token: GtToken<'_>,
    state: ParseState,
    context: &mut GtContext,
) -> GtNodeParseResult<GtAttributeProperty> {
    match state {
        ParseState::Name(span) => {
            let key = GtAttributeKey::parse(token);

            match inner.next() {
                Some(token) => parse(inner, token, ParseState::Value(span, key), context),

                None => Err(GtParseError::UnexpectedEnd(
                    span,
                    GtNode::AttributeProperty,
                    "continuation after identifier",
                )),
            }
        }

        ParseState::Value(span, name) => {
            let value = GtAttributeValue::parse(token, context)?;

            Ok(GtAttributeProperty { span, name, value })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::internal::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_parse() {
        let mut tokens =
            GtParser::parse(GtTokenKind::AttributeProperty, r#"hello = "world""#).unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeProperty::parse(tokens.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtAttributeProperty(
          span: GtSpan(0, 15),
          name: GtAttributeKey(
            span: GtSpan(0, 5),
            value: "hello",
          ),
          value: Literal(GtLiteral(
            span: GtSpan(8, 15),
            doc: None,
            attributes: [],
            value: String("world"),
          )),
        )
        "#
        );
    }
}
