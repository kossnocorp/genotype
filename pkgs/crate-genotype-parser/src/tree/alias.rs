use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GtAlias {
    pub id: GtDefinitionId,
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    #[visit]
    pub name: GtIdentifier,
    #[visit]
    pub descriptor: GtDescriptor,
}

impl GtAlias {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = pair.as_span().into();
        let annotation = context.take_annotation_or_default();
        let mut inner = pair.into_inner();

        let pair = inner
            .next()
            .ok_or_else(|| GtParseError::Internal(span, GtNode::Alias))?;
        let alias = parse(
            inner,
            pair,
            context,
            ParseState::Annotation(span, annotation),
        )?;

        context.exit_parent(span, GtNode::Alias)?;

        Ok(alias)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    context: &mut GtContext,
    state: ParseState,
) -> GtNodeParseResult<GtAlias> {
    match state {
        ParseState::Annotation(span, (doc_acc, mut attributes)) => match pair.as_rule() {
            Rule::line_doc => {
                let doc = pair.into_inner().find(|p| p.as_rule() == Rule::doc);
                let doc_acc = if let Some(pair) = doc {
                    Some(if let Some(doc) = doc_acc {
                        doc.concat(pair)
                    } else {
                        pair.into()
                    })
                } else {
                    doc_acc
                };

                match inner.next() {
                    Some(pair) => parse(
                        inner,
                        pair,
                        context,
                        ParseState::Annotation(span, (doc_acc, attributes)),
                    ),
                    None => Err(GtParseError::Internal(span, GtNode::Alias)),
                }
            }

            Rule::attribute => {
                let attribute = GtAttribute::parse(pair, context)?;
                attributes.push(attribute);

                match inner.next() {
                    Some(pair) => parse(
                        inner,
                        pair,
                        context,
                        ParseState::Annotation(span, (doc_acc, attributes)),
                    ),
                    None => Err(GtParseError::Internal(span, GtNode::Alias)),
                }
            }

            _ => parse(
                inner,
                pair,
                context,
                ParseState::Name(span, (doc_acc, attributes)),
            ),
        },

        ParseState::Name(span, annotation) => {
            let name: GtIdentifier = pair.into();

            context.resolve.exports.push(name.clone());
            context.enter_parent(GtContextParent::Alias(name.clone()));

            match inner.next() {
                Some(pair) => parse(
                    inner,
                    pair,
                    context,
                    ParseState::Descriptor(span, annotation, name),
                ),
                None => Err(GtParseError::Internal(span, GtNode::Alias)),
            }
        }

        ParseState::Descriptor(span, (doc, attributes), name) => {
            let id = context.module_id.definition_id(&name);
            let descriptor = GtDescriptor::parse(pair, context)?;
            Ok(GtAlias {
                id,
                span,
                doc,
                attributes,
                name,
                descriptor,
            })
        }
    }
}

enum ParseState {
    Annotation(GtSpan, GtContextAnnotation),
    Name(GtSpan, GtContextAnnotation),
    Descriptor(GtSpan, GtContextAnnotation, GtIdentifier),
}

#[cfg(test)]
mod tests {
    use crate::test::*;
    use crate::*;
    use insta::assert_ron_snapshot;
    use miette::NamedSource;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::alias, "Hello: { world: string }").unwrap();
        assert_ron_snapshot!(
            GtAlias::parse(pairs.next().unwrap(), &mut GtContext::new("module".into())).unwrap(),
            @r#"
        GtAlias(
          id: GtDefinitionId(GtModuleId("module"), "Hello"),
          span: GtSpan(0, 24),
          doc: None,
          attributes: [],
          name: GtIdentifier(GtSpan(0, 5), "Hello"),
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
        let source_code = NamedSource::new("module.type", "Hello: string".into());
        let parse = GtModule::parse("module".into(), source_code).unwrap();
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
        let mut pairs = GenotypeParser::parse(Rule::alias, "Hello: { world: string }").unwrap();
        let parents = vec![GtContextParent::Alias(GtIdentifier::new(
            (0, 5).into(),
            "Hello".into(),
        ))];
        let mut context = GtContext {
            module_id: "module".into(),
            parents: parents.clone(),
            resolve: GtModuleResolve::new(),
            claimed_names: Default::default(),
            annotation: None,
        };

        GtAlias::parse(pairs.next().unwrap(), &mut context).unwrap();

        assert_eq!(context.parents, parents);
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
            parse_node!(GtAlias, (to_parse_rules(Rule::alias, "Hello: string"), &mut context)),
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
                    to_parse_rules(
                        Rule::alias,
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
}
