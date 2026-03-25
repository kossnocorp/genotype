use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GTAlias {
    pub id: GTDefinitionId,
    pub span: GTSpan,
    #[visit]
    pub doc: Option<GTDoc>,
    #[visit]
    pub attributes: Vec<GTAttribute>,
    #[visit]
    pub name: GTIdentifier,
    #[visit]
    pub descriptor: GTDescriptor,
}

impl GTAlias {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();
        let annotation = context.take_annotation_or_default();
        let mut inner = pair.into_inner();

        let pair = inner
            .next()
            .ok_or_else(|| GTParseError::Internal(span.clone(), GTNode::Alias))?;
        let alias = parse(
            inner,
            pair,
            context,
            ParseState::Annotation(span.clone(), annotation),
        )?;

        context.exit_parent(span, GTNode::Alias)?;

        Ok(alias)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    context: &mut GTContext,
    state: ParseState,
) -> GTNodeParseResult<GTAlias> {
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
                    None => Err(GTParseError::Internal(span, GTNode::Alias)),
                }
            }

            Rule::attribute => {
                let attribute = GTAttribute::parse(pair, context)?;
                attributes.push(attribute);

                match inner.next() {
                    Some(pair) => parse(
                        inner,
                        pair,
                        context,
                        ParseState::Annotation(span, (doc_acc, attributes)),
                    ),
                    None => Err(GTParseError::Internal(span, GTNode::Alias)),
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
            let name: GTIdentifier = pair.into();

            context.resolve.exports.push(name.clone());
            context.enter_parent(GTContextParent::Alias(name.clone()));

            match inner.next() {
                Some(pair) => parse(
                    inner,
                    pair,
                    context,
                    ParseState::Descriptor(span, annotation, name),
                ),
                None => Err(GTParseError::Internal(span, GTNode::Alias)),
            }
        }

        ParseState::Descriptor(span, (doc, attributes), name) => {
            let id = context.module_id.definition_id(&name);
            let descriptor = GTDescriptor::parse(pair, context)?;
            Ok(GTAlias {
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
    Annotation(GTSpan, GTContextAnnotation),
    Name(GTSpan, GTContextAnnotation),
    Descriptor(GTSpan, GTContextAnnotation, GTIdentifier),
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
            GTAlias::parse(pairs.next().unwrap(), &mut GTContext::new("module".into())).unwrap(),
            @r#"
        GTAlias(
          id: GTDefinitionId(GTModuleId("module"), "Hello"),
          span: GTSpan(0, 24),
          doc: None,
          attributes: [],
          name: GTIdentifier(GTSpan(0, 5), "Hello"),
          descriptor: Object(GTObject(
            span: GTSpan(7, 24),
            doc: None,
            attributes: [],
            name: Named(GTIdentifier(GTSpan(0, 5), "Hello")),
            extensions: [],
            properties: [
              GTProperty(
                span: GTSpan(9, 22),
                doc: None,
                attributes: [],
                name: GTKey(GTSpan(9, 14), "world"),
                descriptor: Primitive(GTPrimitive(
                  span: GTSpan(16, 22),
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
        let parse = GTModule::parse("module".into(), source_code).unwrap();
        assert_ron_snapshot!(
            parse.resolve.exports,
            @r#"
        [
          GTIdentifier(GTSpan(0, 5), "Hello"),
        ]
        "#
        );
    }

    #[test]
    fn test_parse_parent() {
        let mut pairs = GenotypeParser::parse(Rule::alias, "Hello: { world: string }").unwrap();
        let parents = vec![GTContextParent::Alias(GTIdentifier::new(
            (0, 5).into(),
            "Hello".into(),
        ))];
        let mut context = GTContext {
            module_id: "module".into(),
            parents: parents.clone(),
            resolve: GTModuleResolve::new(),
            claimed_names: Default::default(),
            annotation: None,
        };

        GTAlias::parse(pairs.next().unwrap(), &mut context).unwrap();

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
            parse_node!(GTAlias, (to_parse_rules(Rule::alias, "Hello: string"), &mut context)),
            @r#"
        GTAlias(
          id: GTDefinitionId(GTModuleId("module"), "Hello"),
          span: GTSpan(0, 13),
          doc: Some(GTDoc(GTSpan(0, 0), "Hello, world!")),
          attributes: [
            GTAttribute(
              span: GTSpan(0, 2),
              name: GTAttributeName(
                span: GTSpan(0, 0),
                value: "example",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(0, 0),
                value: Literal(GTLiteral(
                  span: GTSpan(0, 0),
                  doc: None,
                  attributes: [],
                  value: String("value"),
                )),
              ))),
            ),
          ],
          name: GTIdentifier(GTSpan(0, 5), "Hello"),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(7, 13),
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
                GTAlias,
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
        GTAlias(
          id: GTDefinitionId(GTModuleId("module"), "Hello"),
          span: GTSpan(0, 48),
          doc: Some(GTDoc(GTSpan(0, 34), "Outside\nAlias doc")),
          attributes: [
            GTAttribute(
              span: GTSpan(0, 2),
              name: GTAttributeName(
                span: GTSpan(0, 0),
                value: "outside",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(0, 0),
                value: Literal(GTLiteral(
                  span: GTSpan(0, 0),
                  doc: None,
                  attributes: [],
                  value: String("first"),
                )),
              ))),
            ),
            GTAttribute(
              span: GTSpan(0, 20),
              name: GTAttributeName(
                span: GTSpan(2, 8),
                value: "inside",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(9, 19),
                value: Literal(GTLiteral(
                  span: GTSpan(11, 19),
                  doc: None,
                  attributes: [],
                  value: String("second"),
                )),
              ))),
            ),
          ],
          name: GTIdentifier(GTSpan(35, 40), "Hello"),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(42, 48),
            kind: String,
            doc: None,
            attributes: [],
          )),
        )
        "#
        );
    }
}
