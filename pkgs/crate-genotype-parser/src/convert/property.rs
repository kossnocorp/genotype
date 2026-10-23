use crate::prelude::internal::*;

impl GtParse for GtProperty {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = token.span;
        let required = token.kind == GtTokenKind::RequiredProperty;
        let mut inner = token.into_semantic_children();

        let token = inner.next().ok_or(GtParseError::UnexpectedEnd(
            span,
            GtNode::Property,
            "property name",
        ))?;
        let property = parse(inner, token, ParseState::Doc(span, required, None), context)?;

        context.exit_named_parent(span, GtNode::Property)?;

        Ok(property)
    }
}

enum ParseState {
    Doc(GtSpan, bool, Option<GtDoc>),
    Attributes(GtSpan, bool, Option<GtDoc>, Vec<GtAttribute>),
    Name(GtSpan, bool, Option<GtDoc>, Vec<GtAttribute>),
    Descriptor(GtSpan, bool, Option<GtDoc>, Vec<GtAttribute>, GtKey),
}

fn parse(
    mut inner: GtTokens<'_>,
    token: GtToken<'_>,
    state: ParseState,
    context: &mut GtContext,
) -> GtNodeParseResult<GtProperty> {
    match state {
        ParseState::Doc(span, required, doc_acc) => match token.kind {
            GtTokenKind::LineDoc => {
                let doc = token
                    .into_semantic_children()
                    .find(|p| p.kind == GtTokenKind::Doc);
                let doc_acc = if let Some(token) = doc {
                    Some(if let Some(doc) = doc_acc {
                        concat_doc(&doc, token)
                    } else {
                        GtDoc::parse(token)
                    })
                } else {
                    doc_acc
                };

                match inner.next() {
                    Some(token) => parse(
                        inner,
                        token,
                        ParseState::Doc(span, required, doc_acc),
                        context,
                    ),
                    None => Err(GtParseError::UnexpectedEnd(
                        span,
                        GtNode::Property,
                        "property after doc",
                    )),
                }
            }

            _ => parse(
                inner,
                token,
                ParseState::Attributes(span, required, doc_acc, vec![]),
                context,
            ),
        },

        ParseState::Attributes(span, required, doc, mut attributes) => match token.kind {
            GtTokenKind::Attribute => {
                let attribute = GtAttribute::parse(token, context)?;
                attributes.push(attribute);

                match inner.next() {
                    Some(token) => parse(
                        inner,
                        token,
                        ParseState::Attributes(span, required, doc, attributes),
                        context,
                    ),
                    None => Err(GtParseError::UnexpectedEnd(
                        span,
                        GtNode::Property,
                        "property after attribute",
                    )),
                }
            }

            _ => parse(
                inner,
                token,
                ParseState::Name(span, required, doc, attributes),
                context,
            ),
        },

        ParseState::Name(span, required, doc, attributes) => {
            let name = GtKey::parse(token);

            context.enter_named_parent(GtContextParent::Property(name.clone()));

            match inner.next() {
                Some(token) => parse(
                    inner,
                    token,
                    ParseState::Descriptor(span, required, doc, attributes, name),
                    context,
                ),
                None => Err(GtParseError::UnexpectedEnd(
                    span,
                    GtNode::Property,
                    "property descriptor",
                )),
            }
        }

        ParseState::Descriptor(span, required, doc, attributes, name) => {
            let descriptor = GtDescriptor::parse(token, context)?;
            Ok(GtProperty {
                span,
                doc,
                attributes,
                name,
                descriptor,
                required,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::internal::*;
    use insta::assert_ron_snapshot;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut tokens = GtParser::parse(GtTokenKind::RequiredProperty, "world: string").unwrap();
        assert_ron_snapshot!(
            GtProperty::parse(tokens.next().unwrap(), &mut GtContext::new("module".into())).unwrap(),
            @r#"
        GtProperty(
          span: GtSpan(0, 13),
          doc: None,
          attributes: [],
          name: GtKey(GtSpan(0, 5), "world"),
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(7, 13),
            kind: String,
            doc: None,
            attributes: [],
          )),
          required: true,
        )
        "#
        );
    }

    #[test]
    fn test_parse_parent() {
        let mut tokens = GtParser::parse(GtTokenKind::RequiredProperty, "world: string").unwrap();
        let parents = vec![GtContextParent::Alias(GtIdentifier::new(
            (0, 5).into(),
            "Hello".into(),
        ))];
        let mut context = GtContext {
            module_id: "module".into(),
            named_parents: parents.clone(),
            annotation: None,
            ..Default::default()
        };

        GtProperty::parse(tokens.next().unwrap(), &mut context).unwrap();

        assert_eq!(context.named_parents, parents);
    }
}
