use crate::prelude::internal::*;

impl GTReference {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTParseError> {
        let span: GTSpan = pair.as_span().into();
        let identifier: GTIdentifier = pair.into();
        let (doc, attributes) = context.take_annotation_or_default();

        context.resolve.references.insert(identifier.clone());

        Ok(GTReference {
            span: span.clone(),
            doc,
            attributes,
            id: GTReferenceId(context.module_id.clone(), span),
            definition_id: GTReferenceDefinitionId::Unresolved,
            identifier,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test::*;
    use crate::*;
    use insta::assert_ron_snapshot;
    use miette::NamedSource;

    #[test]
    fn test_parse_references() {
        let parse = GTModule::parse(
            "module".into(),
            NamedSource::new(
                "module.type",
                r#"use user/User

            Author: {
              name: Name,
              user: User
            }

            Name: string"#
                    .into(),
            ),
        )
        .unwrap();
        assert_ron_snapshot!(
            parse.resolve.references,
            @r#"
        [
          GTIdentifier(GTSpan(57, 61), "Name"),
          GTIdentifier(GTSpan(83, 87), "User"),
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
            parse_node!(GTReference, (to_parse_rules(Rule::name, "Hello"), &mut context)),
            @r#"
        GTReference(
          span: GTSpan(0, 5),
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
          id: GTReferenceId(GTModuleId("module"), GTSpan(0, 5)),
          definition_id: Unresolved,
          identifier: GTIdentifier(GTSpan(0, 5), "Hello"),
        )
        "#
        );
    }
}
