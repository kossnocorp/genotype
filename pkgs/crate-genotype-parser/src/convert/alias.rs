use crate::prelude::internal::*;

impl GtParse for GtAlias {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = token.span;
        let annotation = context.take_annotation_or_default();
        let mut inner = token.into_semantic_children();

        let token = inner.next().ok_or(GtParseError::UnexpectedEnd(
            span,
            GtNode::Alias,
            "alias inner",
        ))?;

        let alias = parse(
            inner,
            token,
            context,
            ParseState::Annotation(span, annotation),
        )?;

        context.exit_named_parent(span, GtNode::Alias)?;
        context.exit_generics_scope(span, GtNode::Alias)?;

        Ok(alias)
    }
}

enum ParseState {
    Annotation(GtSpan, GtContextAnnotation),
    Name(GtSpan, GtContextAnnotation),
    Generics(GtSpan, GtContextAnnotation, GtIdentifier),
    Descriptors(
        GtSpan,
        GtContextAnnotation,
        GtIdentifier,
        Vec<GtGenericParameter>,
    ),
}

fn parse(
    mut inner: GtTokens<'_>,
    token: GtToken<'_>,
    context: &mut GtContext,
    state: ParseState,
) -> GtNodeParseResult<GtAlias> {
    match state {
        ParseState::Annotation(span, (doc_acc, mut attributes)) => match token.kind {
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
                        context,
                        ParseState::Annotation(span, (doc_acc, attributes)),
                    ),

                    None => Err(GtParseError::UnexpectedEnd(
                        span,
                        GtNode::Alias,
                        "continuation after doc",
                    )),
                }
            }

            GtTokenKind::Attribute => {
                let attribute = GtAttribute::parse(token, context)?;
                attributes.push(attribute);

                match inner.next() {
                    Some(token) => parse(
                        inner,
                        token,
                        context,
                        ParseState::Annotation(span, (doc_acc, attributes)),
                    ),

                    None => Err(GtParseError::UnexpectedEnd(
                        span,
                        GtNode::Alias,
                        "continuation after attribute",
                    )),
                }
            }

            _ => parse(
                inner,
                token,
                context,
                ParseState::Name(span, (doc_acc, attributes)),
            ),
        },

        ParseState::Name(span, annotation) => {
            let name: GtIdentifier = GtIdentifier::parse(token);

            context.resolve.exports.push(name.clone());
            context.enter_named_parent(GtContextParent::Alias(name.clone()));

            match inner.next() {
                Some(token) => match token.kind {
                    GtTokenKind::GenericParameters => parse(
                        inner,
                        token,
                        context,
                        ParseState::Generics(span, annotation, name),
                    ),

                    GtTokenKind::Descriptors => parse(
                        inner,
                        token,
                        context,
                        ParseState::Descriptors(span, annotation, name, vec![]),
                    ),

                    kind => Err(GtParseError::UnexpectedToken(
                        span,
                        GtNode::Alias,
                        kind,
                        "expected generics or descriptor",
                    )),
                },

                None => Err(GtParseError::UnexpectedEnd(
                    span,
                    GtNode::Alias,
                    "continuation after identifier",
                )),
            }
        }

        ParseState::Generics(span, annotation, name) => {
            let generics = parse_generics(token, context)?;

            match inner.next() {
                Some(token) => parse(
                    inner,
                    token,
                    context,
                    ParseState::Descriptors(span, annotation, name, generics),
                ),

                None => Err(GtParseError::UnexpectedEnd(
                    span,
                    GtNode::Alias,
                    "continuation after generics",
                )),
            }
        }

        ParseState::Descriptors(span, (doc, attributes), name, generics) => {
            context.enter_generics_scope(&generics);

            let id = GtDefinitionId(context.module_id.clone(), name.1.clone());
            let descriptor = GtDescriptor::parse(token, context)?;
            Ok(GtAlias {
                id,
                span,
                doc,
                attributes,
                name,
                generics,
                descriptor,
            })
        }
    }
}

fn parse_generics(
    token: GtToken<'_>,
    context: &mut GtContext,
) -> GtNodeParseResult<Vec<GtGenericParameter>> {
    let inner = token.into_semantic_children();

    let mut generics = vec![];

    for generics_token in inner {
        generics.push(GtGenericParameter::parse(generics_token, context)?);
    }

    Ok(generics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut tokens = GtParser::parse(GtTokenKind::Alias, "Hello: { world: string }").unwrap();
        assert_ron_snapshot!(
            GtAlias::parse(tokens.next().unwrap(), &mut GtContext::new("module".into())).unwrap(),
            @r#"
        GtAlias(
          id: GtDefinitionId(GtModuleId("module"), "Hello"),
          span: GtSpan(0, 24),
          doc: None,
          attributes: [],
          name: GtIdentifier(GtSpan(0, 5), "Hello"),
          generics: [],
          descriptor: Object(GtObject(
            span: GtSpan(7, 24),
            doc: None,
            attributes: [],
            name: Named(GtIdentifier(GtSpan(0, 5), "Hello")),
            extensions: [],
            properties: [
              GtProperty(
                span: GtSpan(9, 22),
                doc: None,
                attributes: [],
                name: GtKey(GtSpan(9, 14), "world"),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(16, 22),
                  kind: String,
                  doc: None,
                  attributes: [],
                )),
                required: true,
              ),
            ],
          )),
        )
        "#
        );
    }

    #[test]
    fn test_parse_exports() {
        let source_code = "Hello: string".to_owned();
        let parse = GtModule::parse("module".into(), &source_code).unwrap();
        assert_ron_snapshot!(
            parse.resolve.exports,
            @r#"
        [
          GtIdentifier(GtSpan(0, 5), "Hello"),
        ]
        "#
        );
    }

    #[test]
    fn test_parse_parent() {
        let mut tokens = GtParser::parse(GtTokenKind::Alias, "Hello: { world: string }").unwrap();
        let parents = vec![GtContextParent::Alias(GtIdentifier::new(
            (0, 5).into(),
            "Hello".into(),
        ))];
        let mut context = GtContext {
            module_id: "module".into(),
            named_parents: parents.clone(),
            ..Default::default()
        };

        GtAlias::parse(tokens.next().unwrap(), &mut context).unwrap();

        assert_eq!(context.named_parents, parents);
    }

    #[test]
    fn test_annotation() {
        let mut context = Gt::context();
        context.provide_annotation((
            Gt::some_doc("Hello, world!"),
            vec![Gt::attribute(
                "example",
                Gt::attribute_assignment(Gt::literal_string("value")),
            )],
        ));
        assert_ron_snapshot!(
            parse_node!(GtAlias, (to_parse_tokens(GtTokenKind::Alias, "Hello: string"), &mut context)),
            @r#"
        GtAlias(
          id: GtDefinitionId(GtModuleId("module"), "Hello"),
          span: GtSpan(0, 13),
          doc: Some(GtDoc(GtSpan(0, 0), "Hello, world!")),
          attributes: [
            GtAttribute(
              span: GtSpan(0, 2),
              name: GtAttributeName(
                span: GtSpan(0, 0),
                value: "example",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(0, 0),
                value: Literal(GtLiteral(
                  span: GtSpan(0, 0),
                  doc: None,
                  attributes: [],
                  value: String("value"),
                )),
              ))),
            ),
          ],
          name: GtIdentifier(GtSpan(0, 5), "Hello"),
          generics: [],
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(7, 13),
            kind: String,
            doc: None,
            attributes: [],
          )),
        )
        "#
        );
    }

    #[test]
    fn test_annotation_merge_and_mixed_order() {
        let mut context = Gt::context();
        context.provide_annotation((
            Gt::some_doc("Outside"),
            vec![Gt::attribute(
                "outside",
                Gt::attribute_assignment(Gt::literal_string("first")),
            )],
        ));
        assert_ron_snapshot!(
            parse_node!(
                GtAlias,
                (
                    to_parse_tokens(
                        GtTokenKind::Alias,
                        indoc! {r#"
                            #[inside = "second"]
                            /// Alias doc
                            Hello: string
                        "#}
                    ),
                    &mut context
                )
            ),
            @r#"
        GtAlias(
          id: GtDefinitionId(GtModuleId("module"), "Hello"),
          span: GtSpan(0, 48),
          doc: Some(GtDoc(GtSpan(0, 34), "Outside\nAlias doc")),
          attributes: [
            GtAttribute(
              span: GtSpan(0, 2),
              name: GtAttributeName(
                span: GtSpan(0, 0),
                value: "outside",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(0, 0),
                value: Literal(GtLiteral(
                  span: GtSpan(0, 0),
                  doc: None,
                  attributes: [],
                  value: String("first"),
                )),
              ))),
            ),
            GtAttribute(
              span: GtSpan(0, 20),
              name: GtAttributeName(
                span: GtSpan(2, 8),
                value: "inside",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(9, 19),
                value: Literal(GtLiteral(
                  span: GtSpan(11, 19),
                  doc: None,
                  attributes: [],
                  value: String("second"),
                )),
              ))),
            ),
          ],
          name: GtIdentifier(GtSpan(35, 40), "Hello"),
          generics: [],
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(42, 48),
            kind: String,
            doc: None,
            attributes: [],
          )),
        )
        "#
        );
    }

    #[test]
    fn test_generics() {
        assert_ron_snapshot!(
            parse_node!(GtAlias, to_parse_args(GtTokenKind::Alias, "List<T>: [T]")),
            @r#"
        GtAlias(
          id: GtDefinitionId(GtModuleId("module"), "List"),
          span: GtSpan(0, 12),
          doc: None,
          attributes: [],
          name: GtIdentifier(GtSpan(0, 4), "List"),
          generics: [
            GtGenericParameter(
              span: GtSpan(5, 6),
              identifier: GtIdentifier(GtSpan(5, 6), "T"),
            ),
          ],
          descriptor: Array(GtArray(
            span: GtSpan(9, 12),
            doc: None,
            attributes: [],
            descriptor: Reference(GtReference(
              span: GtSpan(10, 11),
              doc: None,
              attributes: [],
              id: GtReferenceId(GtModuleId("module"), GtSpan(10, 11)),
              identifier: GtIdentifier(GtSpan(10, 11), "T"),
              arguments: [],
            )),
          )),
        )
        "#
        );
    }
}
