use crate::prelude::internal::*;

impl GtDescriptor {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = pair.as_span().into();

        let mut descriptors = vec![];

        let inner = pair.into_inner();

        let is_union = inner.len() > 1;
        if is_union {
            context.enter_named_parent(GtContextParent::Anonymous);
        }

        for pair in inner {
            let mut descriptor_inner = pair.into_inner();
            let next_pair = descriptor_inner.next().ok_or(GtParseError::UnexpectedEnd(
                span,
                GtNode::Descriptor,
                "descriptor inner",
            ))?;

            let descriptor = parse(
                descriptor_inner,
                next_pair,
                ParseState::Annotation(span, None, vec![]),
                context,
            )?;

            descriptors.push(descriptor);
        }

        if is_union {
            context.exit_named_parent(span, GtNode::Descriptor)?;
        }

        match descriptors.as_slice() {
            [] => Err(GtParseError::Internal(
                span,
                GtNode::Descriptor,
                "no descriptors found",
            )),

            [descriptor] => Ok(descriptor.to_owned()),

            descriptors => Ok(GtDescriptor::Union(GtUnion {
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
    context: &mut GtContext,
) -> GtNodeParseResult<GtDescriptor> {
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
                    None => Err(GtParseError::InternalLegacy(span, GtNode::Descriptor)),
                }
            }

            Rule::attribute => {
                let attribute = GtAttribute::parse(pair, context)?;
                attributes.push(attribute);

                match inner.next() {
                    Some(pair) => parse(
                        inner,
                        pair,
                        ParseState::Annotation(span, doc_acc, attributes),
                        context,
                    ),
                    None => Err(GtParseError::InternalLegacy(span, GtNode::Descriptor)),
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
                Rule::primitive => GtDescriptor::Primitive(GtPrimitive::parse(pair, context)?),

                Rule::reference => GtDescriptor::Reference(GtReference::parse(pair, context)?),

                Rule::object => GtDescriptor::Object(GtObject::parse(pair, context)?),

                Rule::array => GtDescriptor::Array(Box::new(GtArray::parse(pair, context)?)),

                Rule::tuple => GtDescriptor::Tuple(GtTuple::parse(pair, context)?),

                Rule::descriptor => GtDescriptor::parse(pair, context)?,

                Rule::alias => GtDescriptor::Alias(Box::new(GtAlias::parse(pair, context)?)),

                Rule::inline_import => {
                    GtDescriptor::InlineImport(GtInlineImport::parse(pair, context)?)
                }

                Rule::literal => GtDescriptor::Literal(GtLiteral::parse(pair, context)?),

                Rule::record => GtDescriptor::Record(Box::new(GtRecord::parse(pair, context)?)),

                Rule::any => GtDescriptor::Any(GtAny::parse(pair, context)?),

                Rule::branded => GtDescriptor::Branded(GtBranded::parse(pair, context)?),

                rule => {
                    println!("------ Unexpected rule: {:?}", pair);
                    return Err(GtParseError::UnexpectedRule(span, GtNode::Descriptor, rule));
                }
            };

            Ok(descriptor)
        }
    }
}

enum ParseState {
    Annotation(GtSpan, Option<GtDoc>, Vec<GtAttribute>),
    Descriptor(GtSpan, Option<GtDoc>, Vec<GtAttribute>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    pub fn parse_literal() {
        assert_ron_snapshot!(
            parse_node!(
                GtDescriptor,
                to_parse_args(Rule::descriptors, r#""hello""#)
            ),
            @r#"
        Literal(GtLiteral(
          span: GtSpan(0, 7),
          doc: None,
          attributes: [],
          value: String("hello"),
        ))
        "#
        );
        assert_ron_snapshot!(
            parse_node!(
                GtDescriptor,
                to_parse_args(Rule::descriptors, "123")
            ),
            @"
        Literal(GtLiteral(
          span: GtSpan(0, 3),
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
                GtDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Very nice!
                    #[variant = "Hey"]
                    "hello"
                "#})
            ),
            @r#"
        Literal(GtLiteral(
          span: GtSpan(34, 41),
          doc: Some(GtDoc(GtSpan(4, 14), "Very nice!")),
          attributes: [
            GtAttribute(
              span: GtSpan(15, 33),
              name: GtAttributeName(
                span: GtSpan(17, 24),
                value: "variant",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(25, 32),
                value: Literal(GtLiteral(
                  span: GtSpan(27, 32),
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
                GtDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Primitive
                    #[variant = "Hey"]
                    string
                "#})
            ),
            @r#"
        Primitive(GtPrimitive(
          span: GtSpan(33, 39),
          kind: String,
          doc: Some(GtDoc(GtSpan(4, 13), "Primitive")),
          attributes: [
            GtAttribute(
              span: GtSpan(14, 32),
              name: GtAttributeName(
                span: GtSpan(16, 23),
                value: "variant",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(24, 31),
                value: Literal(GtLiteral(
                  span: GtSpan(26, 31),
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
                GtDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Ref
                    #[variant = "Hey"]
                    Hello
                "#})
            ),
            @r#"
        Reference(GtReference(
          span: GtSpan(27, 32),
          doc: Some(GtDoc(GtSpan(4, 7), "Ref")),
          attributes: [
            GtAttribute(
              span: GtSpan(8, 26),
              name: GtAttributeName(
                span: GtSpan(10, 17),
                value: "variant",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(18, 25),
                value: Literal(GtLiteral(
                  span: GtSpan(20, 25),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          id: GtReferenceId(GtModuleId("module"), GtSpan(27, 32)),
          identifier: GtIdentifier(GtSpan(27, 32), "Hello"),
          arguments: [],
        ))
        "#
        );
    }

    #[test]
    pub fn parse_annotated_array() {
        assert_ron_snapshot!(
            parse_node!(
                GtDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Arr
                    #[variant = "Hey"]
                    [string]
                "#})
            ),
            @r#"
        Array(GtArray(
          span: GtSpan(27, 35),
          doc: Some(GtDoc(GtSpan(4, 7), "Arr")),
          attributes: [
            GtAttribute(
              span: GtSpan(8, 26),
              name: GtAttributeName(
                span: GtSpan(10, 17),
                value: "variant",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(18, 25),
                value: Literal(GtLiteral(
                  span: GtSpan(20, 25),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(28, 34),
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
                GtDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Tup
                    #[variant = "Hey"]
                    (string)
                "#})
            ),
            @r#"
        Tuple(GtTuple(
          span: GtSpan(27, 35),
          doc: Some(GtDoc(GtSpan(4, 7), "Tup")),
          attributes: [
            GtAttribute(
              span: GtSpan(8, 26),
              name: GtAttributeName(
                span: GtSpan(10, 17),
                value: "variant",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(18, 25),
                value: Literal(GtLiteral(
                  span: GtSpan(20, 25),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          descriptors: [
            Primitive(GtPrimitive(
              span: GtSpan(28, 34),
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
                GtDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Rec
                    #[variant = "Hey"]
                    { []: string }
                "#})
            ),
            @r#"
        Record(GtRecord(
          span: GtSpan(27, 41),
          doc: Some(GtDoc(GtSpan(4, 7), "Rec")),
          attributes: [
            GtAttribute(
              span: GtSpan(8, 26),
              name: GtAttributeName(
                span: GtSpan(10, 17),
                value: "variant",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(18, 25),
                value: Literal(GtLiteral(
                  span: GtSpan(20, 25),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          key: String(GtSpan(29, 31)),
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(33, 39),
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
                GtDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Import
                    #[variant = "Hey"]
                    ./path/to/module/Name
                "#})
            ),
            @r#"
        InlineImport(GtInlineImport(
          span: GtSpan(30, 51),
          doc: Some(GtDoc(GtSpan(4, 10), "Import")),
          attributes: [
            GtAttribute(
              span: GtSpan(11, 29),
              name: GtAttributeName(
                span: GtSpan(13, 20),
                value: "variant",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(21, 28),
                value: Literal(GtLiteral(
                  span: GtSpan(23, 28),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          name: GtIdentifier(GtSpan(47, 51), "Name"),
          arguments: [],
          path: GtPath(
            span: GtSpan(30, 47),
            id: GtPathModuleId(
              span: GtSpan(30, 47),
              module_id: GtModuleId("module"),
            ),
            path: "./path/to/module",
          ),
        ))
        "#
        );
    }

    #[test]
    pub fn parse_annotated_any() {
        assert_ron_snapshot!(
            parse_node!(
                GtDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Any
                    #[variant = "Hey"]
                    any
                "#})
            ),
            @r#"
        Any(GtAny(
          span: GtSpan(27, 30),
          doc: Some(GtDoc(GtSpan(4, 7), "Any")),
          attributes: [
            GtAttribute(
              span: GtSpan(8, 26),
              name: GtAttributeName(
                span: GtSpan(10, 17),
                value: "variant",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(18, 25),
                value: Literal(GtLiteral(
                  span: GtSpan(20, 25),
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
                GtDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Branded
                    #[variant = "Hey"]
                    @int
                "#})
            ),
            @r#"
        Branded(GtBranded(
          span: GtSpan(31, 35),
          doc: Some(GtDoc(GtSpan(4, 11), "Branded")),
          attributes: [
            GtAttribute(
              span: GtSpan(12, 30),
              name: GtAttributeName(
                span: GtSpan(14, 21),
                value: "variant",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(22, 29),
                value: Literal(GtLiteral(
                  span: GtSpan(24, 29),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          id: GtDefinitionId(GtModuleId("module"), "I64"),
          name: GtIdentifier(GtSpan(31, 35), "I64"),
          primitive: GtPrimitive(
            span: GtSpan(32, 35),
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
                GtDescriptor,
                to_parse_args(Rule::descriptors, indoc! {r#"
                    /// Outer
                    #[outer = "Yes"]
                    #[inner = "No"]
                    /// Inner
                    Hello: string
                "#})
            ),
            @r#"
        Alias(GtAlias(
          id: GtDefinitionId(GtModuleId("module"), "Hello"),
          span: GtSpan(53, 66),
          doc: Some(GtDoc(GtSpan(4, 52), "Outer\nInner")),
          attributes: [
            GtAttribute(
              span: GtSpan(10, 26),
              name: GtAttributeName(
                span: GtSpan(12, 17),
                value: "outer",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(18, 25),
                value: Literal(GtLiteral(
                  span: GtSpan(20, 25),
                  doc: None,
                  attributes: [],
                  value: String("Yes"),
                )),
              ))),
            ),
            GtAttribute(
              span: GtSpan(27, 42),
              name: GtAttributeName(
                span: GtSpan(29, 34),
                value: "inner",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(35, 41),
                value: Literal(GtLiteral(
                  span: GtSpan(37, 41),
                  doc: None,
                  attributes: [],
                  value: String("No"),
                )),
              ))),
            ),
          ],
          name: GtIdentifier(GtSpan(53, 58), "Hello"),
          generics: [],
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(60, 66),
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
        context.enter_named_parent(GtContextParent::Alias(Gt::identifier("Hello")));

        assert_ron_snapshot!(
            parse_node!(
                GtDescriptor,
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
        Object(GtObject(
          span: GtSpan(27, 44),
          doc: Some(GtDoc(GtSpan(4, 7), "Obj")),
          attributes: [
            GtAttribute(
              span: GtSpan(8, 26),
              name: GtAttributeName(
                span: GtSpan(10, 17),
                value: "variant",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(18, 25),
                value: Literal(GtLiteral(
                  span: GtSpan(20, 25),
                  doc: None,
                  attributes: [],
                  value: String("Hey"),
                )),
              ))),
            ),
          ],
          name: Named(GtIdentifier(GtSpan(0, 0), "Hello")),
          extensions: [],
          properties: [
            GtProperty(
              span: GtSpan(29, 42),
              doc: None,
              attributes: [],
              name: GtKey(GtSpan(29, 34), "value"),
              descriptor: Primitive(GtPrimitive(
                span: GtSpan(36, 42),
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
