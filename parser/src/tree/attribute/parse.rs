use crate::prelude::internal::*;

impl GTAttribute {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner
            .next()
            .ok_or_else(|| GTParseError::UnexpectedEnd(span.clone(), GTNode::Attribute))?;

        parse(inner, pair, ParseState::Name(span), context)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
    context: &mut GTContext,
) -> GTNodeParseResult<GTAttribute> {
    match state {
        ParseState::Name(span) => {
            let name_span: GTSpan = pair.as_span().into();
            let name = GTAttributeName::new(name_span.clone(), pair.as_str().into());

            match inner.next() {
                Some(pair) => parse(inner, pair, ParseState::Descriptor(span, name), context),

                None => Ok(GTAttribute {
                    span,
                    name,
                    descriptor: None,
                }),
            }
        }

        ParseState::Descriptor(span, name) => {
            let descriptor = match pair.as_rule() {
                Rule::attribute_descriptor => Some(GTAttributeDescriptor::parse(pair, context)?),
                _ => None,
            };

            Ok(GTAttribute {
                span,
                name,
                descriptor,
            })
        }
    }
}

enum ParseState {
    Name(GTSpan),
    Descriptor(GTSpan, GTAttributeName),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use insta::assert_ron_snapshot;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_simple() {
        let mut pairs = GenotypeParser::parse(Rule::attribute, "#[tag]").unwrap();
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTAttribute::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GTAttribute(
          span: GTSpan(0, 6),
          name: GTAttributeName(
            span: GTSpan(2, 5),
            value: "tag",
          ),
          descriptor: None,
        )
        "#
        );
    }

    #[test]
    fn test_parse_assignment() {
        let mut pairs = GenotypeParser::parse(Rule::attribute, "#[answer = 42]").unwrap();
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTAttribute::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GTAttribute(
          span: GTSpan(0, 14),
          name: GTAttributeName(
            span: GTSpan(2, 8),
            value: "answer",
          ),
          descriptor: Some(Assignment(GTAttributeAssignment(
            span: GTSpan(9, 13),
            value: Literal(GTLiteral(
              span: GTSpan(11, 13),
              doc: None,
              attributes: [],
              value: Integer(42),
            )),
          ))),
        )
        "#
        );
    }

    #[test]
    fn test_parse_arguments() {
        let mut pairs =
            GenotypeParser::parse(Rule::attribute, r#"#[say("hello", "world")]"#).unwrap();
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTAttribute::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GTAttribute(
          span: GTSpan(0, 24),
          name: GTAttributeName(
            span: GTSpan(2, 5),
            value: "say",
          ),
          descriptor: Some(Arguments([
            Literal(GTLiteral(
              span: GTSpan(6, 13),
              doc: None,
              attributes: [],
              value: String("hello"),
            )),
            Literal(GTLiteral(
              span: GTSpan(15, 22),
              doc: None,
              attributes: [],
              value: String("world"),
            )),
          ])),
        )
        "#
        );
    }

    #[test]
    fn test_parse_properties() {
        let mut pairs =
            GenotypeParser::parse(Rule::attribute, r#"#[say(hello = "world", qwe = 123)]"#)
                .unwrap();
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTAttribute::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GTAttribute(
          span: GTSpan(0, 34),
          name: GTAttributeName(
            span: GTSpan(2, 5),
            value: "say",
          ),
          descriptor: Some(Properties([
            GTAttributeProperty(
              span: GTSpan(6, 21),
              name: GTAttributeKey(
                span: GTSpan(6, 11),
                value: "hello",
              ),
              value: Literal(GTLiteral(
                span: GTSpan(14, 21),
                doc: None,
                attributes: [],
                value: String("world"),
              )),
            ),
            GTAttributeProperty(
              span: GTSpan(23, 32),
              name: GTAttributeKey(
                span: GTSpan(23, 26),
                value: "qwe",
              ),
              value: Literal(GTLiteral(
                span: GTSpan(29, 32),
                doc: None,
                attributes: [],
                value: Integer(123),
              )),
            ),
          ])),
        )
        "#
        );
    }
}
