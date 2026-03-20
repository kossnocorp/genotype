use crate::prelude::internal::*;

impl GTAttributeProperty {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner
            .next()
            .ok_or_else(|| GTParseError::UnexpectedEnd(span.clone(), GTNode::AttributeProperty))?;

        let property = parse(inner, pair, ParseState::Name(span), context)?;

        Ok(property)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
    context: &mut GTContext,
) -> GTNodeParseResult<GTAttributeProperty> {
    match state {
        ParseState::Name(span) => {
            let key = GTAttributeKey::parse(pair);

            match inner.next() {
                Some(pair) => parse(inner, pair, ParseState::Value(span, key), context),

                None => Err(GTParseError::UnexpectedEnd(
                    span.clone(),
                    GTNode::AttributeProperty,
                )),
            }
        }

        ParseState::Value(span, name) => {
            let value = GTAttributeValue::parse(pair, context)?;

            Ok(GTAttributeProperty { span, name, value })
        }
    }
}

enum ParseState {
    Name(GTSpan),
    Value(GTSpan, GTAttributeKey),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use insta::assert_ron_snapshot;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs =
            GenotypeParser::parse(Rule::attribute_property, r#"hello = "world""#).unwrap();
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTAttributeProperty::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GTAttributeProperty(
          span: GTSpan(0, 15),
          name: GTAttributeKey(
            span: GTSpan(0, 5),
            value: "hello",
          ),
          value: Literal(GTLiteral(
            span: GTSpan(8, 15),
            doc: None,
            attributes: [],
            value: String("world"),
          )),
        )
        "#
        );
    }
}
