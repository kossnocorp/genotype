use crate::prelude::internal::*;

impl GtParse for GtAttribute {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = token.span;

        let mut inner = token.into_semantic_children();
        let token = inner.next().ok_or(GtParseError::UnexpectedEnd(
            span,
            GtNode::Attribute,
            "attribute inner",
        ))?;

        parse(inner, token, ParseState::Name(span), context)
    }
}

enum ParseState {
    Name(GtSpan),
    Descriptor(GtSpan, GtAttributeName),
}

fn parse(
    mut inner: GtTokens<'_>,
    token: GtToken<'_>,
    state: ParseState,
    context: &mut GtContext,
) -> GtNodeParseResult<GtAttribute> {
    match state {
        ParseState::Name(span) => {
            let name_span: GtSpan = token.span;
            let name = GtAttributeName::new(name_span, token.as_str().into());

            match inner.next() {
                Some(token) => parse(inner, token, ParseState::Descriptor(span, name), context),

                None => Ok(GtAttribute {
                    span,
                    name,
                    descriptor: None,
                }),
            }
        }

        ParseState::Descriptor(span, name) => {
            let descriptor = match token.kind {
                GtTokenKind::AttributeDescriptor => {
                    Some(GtAttributeDescriptor::parse(token, context)?)
                }
                _ => None,
            };

            Ok(GtAttribute {
                span,
                name,
                descriptor,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::internal::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_parse_simple() {
        let mut tokens = GtParser::parse(GtTokenKind::Attribute, "#[tag]").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttribute::parse(tokens.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtAttribute(
          span: GtSpan(0, 6),
          name: GtAttributeName(
            span: GtSpan(2, 5),
            value: "tag",
          ),
          descriptor: None,
        )
        "#
        );
    }

    #[test]
    fn test_parse_assignment() {
        let mut tokens = GtParser::parse(GtTokenKind::Attribute, "#[answer = 42]").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttribute::parse(tokens.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtAttribute(
          span: GtSpan(0, 14),
          name: GtAttributeName(
            span: GtSpan(2, 8),
            value: "answer",
          ),
          descriptor: Some(Assignment(GtAttributeAssignment(
            span: GtSpan(9, 13),
            value: Literal(GtLiteral(
              span: GtSpan(11, 13),
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
        let mut tokens =
            GtParser::parse(GtTokenKind::Attribute, r#"#[say("hello", "world")]"#).unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttribute::parse(tokens.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtAttribute(
          span: GtSpan(0, 24),
          name: GtAttributeName(
            span: GtSpan(2, 5),
            value: "say",
          ),
          descriptor: Some(Arguments([
            Literal(GtLiteral(
              span: GtSpan(6, 13),
              doc: None,
              attributes: [],
              value: String("hello"),
            )),
            Literal(GtLiteral(
              span: GtSpan(15, 22),
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
        let mut tokens = GtParser::parse(
            GtTokenKind::Attribute,
            r#"#[say(hello = "world", qwe = 123)]"#,
        )
        .unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttribute::parse(tokens.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtAttribute(
          span: GtSpan(0, 34),
          name: GtAttributeName(
            span: GtSpan(2, 5),
            value: "say",
          ),
          descriptor: Some(Properties([
            GtAttributeProperty(
              span: GtSpan(6, 21),
              name: GtAttributeKey(
                span: GtSpan(6, 11),
                value: "hello",
              ),
              value: Literal(GtLiteral(
                span: GtSpan(14, 21),
                doc: None,
                attributes: [],
                value: String("world"),
              )),
            ),
            GtAttributeProperty(
              span: GtSpan(23, 32),
              name: GtAttributeKey(
                span: GtSpan(23, 26),
                value: "qwe",
              ),
              value: Literal(GtLiteral(
                span: GtSpan(29, 32),
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
