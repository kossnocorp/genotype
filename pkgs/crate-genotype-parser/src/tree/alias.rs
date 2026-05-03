use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
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
    pub generics: Vec<GtGenericParameter>,
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
            .ok_or_else(|| GtParseError::UnexpectedEnd(span, GtNode::Alias, "alias inner"))?;

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

                    None => Err(GtParseError::UnexpectedEnd(
                        span,
                        GtNode::Alias,
                        "continuation after doc",
                    )),
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

                    None => Err(GtParseError::UnexpectedEnd(
                        span,
                        GtNode::Alias,
                        "continuation after attribute",
                    )),
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
                Some(pair) => match pair.as_rule() {
                    Rule::generic_parameters => parse(
                        inner,
                        pair,
                        context,
                        ParseState::Generics(span, annotation, name),
                    ),

                    Rule::descriptors => parse(
                        inner,
                        pair,
                        context,
                        ParseState::Descriptors(span, annotation, name, vec![]),
                    ),

                    rule => Err(GtParseError::UnexpectedRule(span, GtNode::Alias, rule)),
                },

                None => Err(GtParseError::UnexpectedEnd(
                    span,
                    GtNode::Alias,
                    "continuation after identifier",
                )),
            }
        }

        ParseState::Generics(span, annotation, name) => {
            println!("Parsing generics for alias '{:?}'", &name);
            let generics = parse_generics(pair, context)?;

            match inner.next() {
                Some(pair) => parse(
                    inner,
                    pair,
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
            let id = context.module_id.definition_id(&name);
            let descriptor = GtDescriptor::parse(pair, context)?;
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
    pair: Pair<'_, Rule>,
    context: &mut GtContext,
) -> GtNodeParseResult<Vec<GtGenericParameter>> {
    let mut inner = pair.into_inner();

    let mut generics = vec![];

    while let Some(generics_pair) = inner.next() {
        println!(
            "????? Parsing generic parameter: {:?}",
            generics_pair.as_str()
        );
        generics.push(GtGenericParameter::parse(generics_pair, context)?);
        // for pair in generics_pair.into_inner() {
        //     generics.push(GtGenericParameter::parse(pair, context)?);
        // }
    }

    Ok(generics)
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

#[cfg(test)]
mod tests {
    use super::*;

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
            parse_node!(GtAlias, to_parse_args(Rule::alias, "List<T>: [T]")),
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
