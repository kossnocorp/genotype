use crate::prelude::internal::*;

impl GTDescriptor {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut descriptors = vec![];

        let inner = pair.into_inner();

        let is_union = inner.len() > 1;
        if is_union {
            context.enter_parent(GTContextParent::Anonymous);
        }

        for pair in inner {
            let mut descriptor_inner = pair.into_inner();
            let next_pair = descriptor_inner
                .next()
                .ok_or_else(|| GTParseError::UnexpectedEnd(span.clone(), GTNode::Descriptor))?;

            let descriptor = parse(
                descriptor_inner,
                next_pair,
                ParseState::Annotation(span.clone(), None, vec![]),
                context,
            )?;

            descriptors.push(descriptor);
        }

        if is_union {
            context.exit_parent(span.clone(), GTNode::Descriptor)?;
        }

        match descriptors.as_slice() {
            [] => Err(GTParseError::InternalMessage(
                span,
                GTNode::Descriptor,
                "no descriptors found",
            )),

            [descriptor] => Ok(descriptor.to_owned()),

            descriptors => Ok(GTDescriptor::Union(GTUnion {
                span,
                doc: None,
                attributes: vec![],
                descriptors: descriptors.to_owned(),
            })),
        }
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
    context: &mut GTContext,
) -> GTNodeParseResult<GTDescriptor> {
    match state {
        ParseState::Annotation(span, doc_acc, mut attributes) => match pair.as_rule() {
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
                        ParseState::Annotation(span, doc_acc, attributes),
                        context,
                    ),
                    None => Err(GTParseError::Internal(span, GTNode::Descriptor)),
                }
            }

            Rule::attribute => {
                let attribute = GTAttribute::parse(pair, context)?;
                attributes.push(attribute);

                match inner.next() {
                    Some(pair) => parse(
                        inner,
                        pair,
                        ParseState::Annotation(span, doc_acc, attributes),
                        context,
                    ),
                    None => Err(GTParseError::Internal(span, GTNode::Descriptor)),
                }
            }

            _ => parse(
                inner,
                pair,
                ParseState::Descriptor(span, doc_acc, attributes),
                context,
            ),
        },

        ParseState::Descriptor(span, doc, attributes) => {
            context.provide_annotation((doc, attributes));

            let descriptor = match pair.as_rule() {
                Rule::primitive => GTDescriptor::Primitive(GTPrimitive::parse(pair, context)?),

                Rule::name => GTDescriptor::Reference(GTReference::parse(pair, context)?),

                Rule::object => GTDescriptor::Object(GTObject::parse(pair, context)?),

                Rule::array => GTDescriptor::Array(Box::new(GTArray::parse(pair, context)?)),

                Rule::tuple => GTDescriptor::Tuple(GTTuple::parse(pair, context)?),

                Rule::descriptor => GTDescriptor::parse(pair, context)?,

                Rule::alias => GTDescriptor::Alias(Box::new(GTAlias::parse(pair, context)?)),

                Rule::inline_import => {
                    GTDescriptor::InlineImport(GTInlineImport::parse(pair, context)?)
                }

                Rule::literal => GTDescriptor::Literal(GTLiteral::parse(pair, context)?),

                Rule::record => GTDescriptor::Record(Box::new(GTRecord::parse(pair, context)?)),

                Rule::any => GTDescriptor::Any(GTAny::parse(pair, context)?),

                Rule::branded => GTDescriptor::Branded(GTBranded::parse(pair, context)?),

                rule => {
                    return Err(GTParseError::UnexpectedRule(
                        span.clone(),
                        GTNode::Descriptor,
                        rule,
                    ));
                }
            };

            Ok(descriptor)
        }
    }
}

enum ParseState {
    Annotation(GTSpan, Option<GTDoc>, Vec<GTAttribute>),
    Descriptor(GTSpan, Option<GTDoc>, Vec<GTAttribute>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    pub fn parse_literal() {
        assert_ron_snapshot!(
            parse_node!(
                GTDescriptor,
                to_parse_args(Rule::descriptors, r#""hello""#)
            ),
            @r#"
        Literal(GTLiteral(
          span: GTSpan(0, 7),
          doc: None,
          attributes: [],
          value: String("hello"),
        ))
        "#
        );
        assert_ron_snapshot!(
            parse_node!(
                GTDescriptor,
                to_parse_args(Rule::descriptors, "123")
            ),
            @"
        Literal(GTLiteral(
          span: GTSpan(0, 3),
          doc: None,
          attributes: [],
          value: Integer(123),
        ))
        "
        );
    }

    #[test]
    pub fn parse_annotated_literal() {
        assert_ron_snapshot!(
            parse_node!(
                GTDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Very nice!
                    #[variant = "Hey"]
                    "hello"
                "#})
            ),
            @r#"
        Literal(GTLiteral(
          span: GTSpan(34, 41),
          doc: Some(GTDoc(GTSpan(4, 14), "Very nice!")),
          attributes: [
            GTAttribute(
              span: GTSpan(15, 33),
              name: GTAttributeName(
                span: GTSpan(17, 24),
                value: "variant",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(25, 32),
                value: Literal(GTLiteral(
                  span: GTSpan(27, 32),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          value: String("hello"),
        ))
        "#
        );
    }

    #[test]
    pub fn parse_annotated_primitive() {
        assert_ron_snapshot!(
            parse_node!(
                GTDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Primitive
                    #[variant = "Hey"]
                    string
                "#})
            ),
            @r#"
        Primitive(GTPrimitive(
          span: GTSpan(33, 39),
          kind: String,
          doc: Some(GTDoc(GTSpan(4, 13), "Primitive")),
          attributes: [
            GTAttribute(
              span: GTSpan(14, 32),
              name: GTAttributeName(
                span: GTSpan(16, 23),
                value: "variant",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(24, 31),
                value: Literal(GTLiteral(
                  span: GTSpan(26, 31),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
        ))
        "#
        );
    }

    #[test]
    pub fn parse_annotated_reference() {
        assert_ron_snapshot!(
            parse_node!(
                GTDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Ref
                    #[variant = "Hey"]
                    Hello
                "#})
            ),
            @r#"
        Reference(GTReference(
          span: GTSpan(27, 32),
          doc: Some(GTDoc(GTSpan(4, 7), "Ref")),
          attributes: [
            GTAttribute(
              span: GTSpan(8, 26),
              name: GTAttributeName(
                span: GTSpan(10, 17),
                value: "variant",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(18, 25),
                value: Literal(GTLiteral(
                  span: GTSpan(20, 25),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          id: GTReferenceId(GTModuleId("module"), GTSpan(27, 32)),
          definition_id: Unresolved,
          identifier: GTIdentifier(GTSpan(27, 32), "Hello"),
        ))
        "#
        );
    }

    #[test]
    pub fn parse_annotated_array() {
        assert_ron_snapshot!(
            parse_node!(
                GTDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Arr
                    #[variant = "Hey"]
                    [string]
                "#})
            ),
            @r#"
        Array(GTArray(
          span: GTSpan(27, 35),
          doc: Some(GTDoc(GTSpan(4, 7), "Arr")),
          attributes: [
            GTAttribute(
              span: GTSpan(8, 26),
              name: GTAttributeName(
                span: GTSpan(10, 17),
                value: "variant",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(18, 25),
                value: Literal(GTLiteral(
                  span: GTSpan(20, 25),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(28, 34),
            kind: String,
            doc: None,
            attributes: [],
          )),
        ))
        "#
        );
    }

    #[test]
    pub fn parse_annotated_tuple() {
        assert_ron_snapshot!(
            parse_node!(
                GTDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Tup
                    #[variant = "Hey"]
                    (string)
                "#})
            ),
            @r#"
        Tuple(GTTuple(
          span: GTSpan(27, 35),
          doc: Some(GTDoc(GTSpan(4, 7), "Tup")),
          attributes: [
            GTAttribute(
              span: GTSpan(8, 26),
              name: GTAttributeName(
                span: GTSpan(10, 17),
                value: "variant",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(18, 25),
                value: Literal(GTLiteral(
                  span: GTSpan(20, 25),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          descriptors: [
            Primitive(GTPrimitive(
              span: GTSpan(28, 34),
              kind: String,
              doc: None,
              attributes: [],
            )),
          ],
        ))
        "#
        );
    }

    #[test]
    pub fn parse_annotated_record() {
        assert_ron_snapshot!(
            parse_node!(
                GTDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Rec
                    #[variant = "Hey"]
                    { []: string }
                "#})
            ),
            @r#"
        Record(GTRecord(
          span: GTSpan(27, 41),
          doc: Some(GTDoc(GTSpan(4, 7), "Rec")),
          attributes: [
            GTAttribute(
              span: GTSpan(8, 26),
              name: GTAttributeName(
                span: GTSpan(10, 17),
                value: "variant",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(18, 25),
                value: Literal(GTLiteral(
                  span: GTSpan(20, 25),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          key: String(GTSpan(29, 31)),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(33, 39),
            kind: String,
            doc: None,
            attributes: [],
          )),
        ))
        "#
        );
    }

    #[test]
    pub fn parse_annotated_inline_import() {
        assert_ron_snapshot!(
            parse_node!(
                GTDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Import
                    #[variant = "Hey"]
                    ./path/to/module/Name
                "#})
            ),
            @r#"
        InlineImport(GTInlineImport(
          span: GTSpan(30, 51),
          doc: Some(GTDoc(GTSpan(4, 10), "Import")),
          attributes: [
            GTAttribute(
              span: GTSpan(11, 29),
              name: GTAttributeName(
                span: GTSpan(13, 20),
                value: "variant",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(21, 28),
                value: Literal(GTLiteral(
                  span: GTSpan(23, 28),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          name: GTIdentifier(GTSpan(47, 51), "Name"),
          path: GTPath(GTSpan(30, 46), Unresolved, "./path/to/module"),
        ))
        "#
        );
    }

    #[test]
    pub fn parse_annotated_any() {
        assert_ron_snapshot!(
            parse_node!(
                GTDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Any
                    #[variant = "Hey"]
                    any
                "#})
            ),
            @r#"
        Any(GTAny(
          span: GTSpan(27, 30),
          doc: Some(GTDoc(GTSpan(4, 7), "Any")),
          attributes: [
            GTAttribute(
              span: GTSpan(8, 26),
              name: GTAttributeName(
                span: GTSpan(10, 17),
                value: "variant",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(18, 25),
                value: Literal(GTLiteral(
                  span: GTSpan(20, 25),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
        ))
        "#
        );
    }

    #[test]
    pub fn parse_annotated_branded() {
        assert_ron_snapshot!(
            parse_node!(
                GTDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Branded
                    #[variant = "Hey"]
                    @int
                "#})
            ),
            @r#"
        Branded(GTBranded(
          span: GTSpan(31, 35),
          doc: Some(GTDoc(GTSpan(4, 11), "Branded")),
          attributes: [
            GTAttribute(
              span: GTSpan(12, 30),
              name: GTAttributeName(
                span: GTSpan(14, 21),
                value: "variant",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(22, 29),
                value: Literal(GTLiteral(
                  span: GTSpan(24, 29),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          id: GTDefinitionId(GTModuleId("module"), "I64"),
          name: GTIdentifier(GTSpan(31, 35), "I64"),
          primitive: GTPrimitive(
            span: GTSpan(32, 35),
            kind: Int64,
            doc: None,
            attributes: [],
          ),
        ))
        "#
        );
    }

    #[test]
    pub fn parse_annotated_alias() {
        assert_ron_snapshot!(
            parse_node!(
                GTDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Outer
                    #[outer = "Yes"]
                    #[inner = "No"]
                    /// Inner
                    Hello: string
                "#})
            ),
            @r#"
        Alias(GTAlias(
          id: GTDefinitionId(GTModuleId("module"), "Hello"),
          span: GTSpan(53, 66),
          doc: Some(GTDoc(GTSpan(4, 52), "Outer\nInner")),
          attributes: [
            GTAttribute(
              span: GTSpan(10, 26),
              name: GTAttributeName(
                span: GTSpan(12, 17),
                value: "outer",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(18, 25),
                value: Literal(GTLiteral(
                  span: GTSpan(20, 25),
                  doc: None,
                  attributes: [],
                  value: String("Yes"),
                )),
              ))),
            ),
            GTAttribute(
              span: GTSpan(27, 42),
              name: GTAttributeName(
                span: GTSpan(29, 34),
                value: "inner",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(35, 41),
                value: Literal(GTLiteral(
                  span: GTSpan(37, 41),
                  doc: None,
                  attributes: [],
                  value: String("No"),
                )),
              ))),
            ),
          ],
          name: GTIdentifier(GTSpan(53, 58), "Hello"),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(60, 66),
            kind: String,
            doc: None,
            attributes: [],
          )),
        ))
        "#
        );
    }

    #[test]
    pub fn parse_annotated_object() {
        let mut context = Gt::context();
        context.enter_parent(GTContextParent::Alias(Gt::identifier("Hello")));

        assert_ron_snapshot!(
            parse_node!(
                GTDescriptor,
                (
                    to_parse_rules(
                        Rule::descriptors,
                        indoc! {r#"
                            /// Obj
                            #[variant = "Hey"]
                            { value: string }
                        "#}
                    ),
                    &mut context
                )
            ),
            @r#"
        Object(GTObject(
          span: GTSpan(27, 44),
          doc: Some(GTDoc(GTSpan(4, 7), "Obj")),
          attributes: [
            GTAttribute(
              span: GTSpan(8, 26),
              name: GTAttributeName(
                span: GTSpan(10, 17),
                value: "variant",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(18, 25),
                value: Literal(GTLiteral(
                  span: GTSpan(20, 25),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          name: Named(GTIdentifier(GTSpan(0, 0), "Hello")),
          extensions: [],
          properties: [
            GTProperty(
              span: GTSpan(29, 42),
              doc: None,
              attributes: [],
              name: GTKey(GTSpan(29, 34), "value"),
              descriptor: Primitive(GTPrimitive(
                span: GTSpan(36, 42),
                kind: String,
                doc: None,
                attributes: [],
              )),
              required: true,
            ),
          ],
        ))
        "#
        );
    }
}
