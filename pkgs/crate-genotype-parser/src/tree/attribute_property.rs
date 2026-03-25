use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtAttributeProperty {
    pub span: GtSpan,
    #[visit]
    pub name: GtAttributeKey,
    pub value: GtAttributeValue,
}

impl GtAttributeProperty {
    pub fn new(span: GtSpan, name: GtAttributeKey, value: GtAttributeValue) -> Self {
        Self { span, name, value }
    }
}

impl GtAttributeProperty {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner
            .next()
            .ok_or_else(|| GtParseError::UnexpectedEnd(span.clone(), GtNode::AttributeProperty))?;

        let property = parse(inner, pair, ParseState::Name(span), context)?;

        Ok(property)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
    context: &mut GtContext,
) -> GtNodeParseResult<GtAttributeProperty> {
    match state {
        ParseState::Name(span) => {
            let key = GtAttributeKey::parse(pair);

            match inner.next() {
                Some(pair) => parse(inner, pair, ParseState::Value(span, key), context),

                None => Err(GtParseError::UnexpectedEnd(
                    span.clone(),
                    GtNode::AttributeProperty,
                )),
            }
        }

        ParseState::Value(span, name) => {
            let value = GtAttributeValue::parse(pair, context)?;

            Ok(GtAttributeProperty { span, name, value })
        }
    }
}

enum ParseState {
    Name(GtSpan),
    Value(GtSpan, GtAttributeKey),
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
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeProperty::parse(pairs.next().unwrap(), &mut context).unwrap(),
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
