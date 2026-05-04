use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtReference {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    pub id: GtReferenceId,
    #[visit]
    pub identifier: GtIdentifier,
    #[visit]
    pub arguments: Vec<GtGenericArgument>,
}

impl GtReference {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = pair.as_span().into();
        let (identifier, arguments) = GtReference::parse_name_with_arguments(pair, context)?;
        let (doc, attributes) = context.take_annotation_or_default();

        context.resolve.references.insert(identifier.clone());
        context.resolve_reference_identifier_as_generic_parameter(&identifier);

        Ok(GtReference {
            span,
            doc,
            attributes,
            id: GtReferenceId(context.module_id.clone(), span),
            identifier,
            arguments,
        })
    }

    pub fn parse_name_with_arguments(
        pair: Pair<'_, Rule>,
        context: &mut GtContext,
    ) -> Result<(GtIdentifier, Vec<GtGenericArgument>), GtParseError> {
        let span: GtSpan = pair.as_span().into();
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or(GtParseError::UnexpectedEnd(
                span,
                GtNode::Reference,
                "reference inner",
            ))?
            .into();

        let mut arguments = vec![];
        while let Some(arguments_pair) = inner.next() {
            arguments.push(GtGenericArgument::parse(arguments_pair, context)?);
        }

        Ok((name, arguments))
    }
}

#[cfg(test)]
mod tests {
    use crate::test::*;
    use crate::*;

    #[test]
    fn test_parse_references() {
        let parse = GtModule::parse(
            "module".into(),
            &r#"use user/User

            Author: {
              name: Name,
              user: User
            }

            Name: string"#
                .to_owned(),
        )
        .unwrap();
        assert_ron_snapshot!(
            parse.resolve.references,
            @r#"
        [
          GtIdentifier(GtSpan(57, 61), "Name"),
          GtIdentifier(GtSpan(83, 87), "User"),
        ]
        "#
        );
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
            parse_node!(GtReference, (to_parse_rules(Rule::reference, "Hello"), &mut context)),
            @r#"
        GtReference(
          span: GtSpan(0, 5),
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
          id: GtReferenceId(GtModuleId("module"), GtSpan(0, 5)),
          identifier: GtIdentifier(GtSpan(0, 5), "Hello"),
          arguments: [],
        )
        "#
        );
    }

    #[test]
    fn test_arguments() {
        assert_ron_snapshot!(
            parse_node!(GtReference, to_parse_args(Rule::reference, "Message<string>")),
            @r#"
        GtReference(
          span: GtSpan(0, 15),
          doc: None,
          attributes: [],
          id: GtReferenceId(GtModuleId("module"), GtSpan(0, 15)),
          identifier: GtIdentifier(GtSpan(0, 7), "Message"),
          arguments: [
            GtGenericArgument(
              span: GtSpan(7, 15),
              descriptor: Primitive(GtPrimitive(
                span: GtSpan(8, 14),
                kind: String,
                doc: None,
                attributes: [],
              )),
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_identified_generic_parameters() {
        let mut context = Gt::context();
        context.enter_generics_scope(&vec![Gt::generic_parameter("T")]);
        assert_ron_snapshot!(
            parse_node!(GtReference, (to_parse_rules(Rule::reference, "T"), &mut context)),
            @r#"
        GtReference(
          span: GtSpan(0, 1),
          doc: None,
          attributes: [],
          id: GtReferenceId(GtModuleId("module"), GtSpan(0, 1)),
          identifier: GtIdentifier(GtSpan(0, 1), "T"),
          arguments: [],
        )
        "#
        );

        assert_ron_snapshot!(
            context.resolve.generic_parameters,
            @r#"
        [
          GtIdentifier(GtSpan(0, 1), "T"),
        ]
        "#
        );
    }
}
