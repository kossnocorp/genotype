use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtReference {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    pub id: GtReferenceId,
    pub definition_id: GtReferenceDefinitionId,
    #[visit]
    pub identifier: GtIdentifier,
}

impl GtReference {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = pair.as_span().into();
        let identifier: GtIdentifier = pair.into();
        let (doc, attributes) = context.take_annotation_or_default();

        context.resolve.references.insert(identifier.clone());

        Ok(GtReference {
            span: span.clone(),
            doc,
            attributes,
            id: GtReferenceId(context.module_id.clone(), span),
            definition_id: GtReferenceDefinitionId::Unresolved,
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
        let parse = GtModule::parse(
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
            parse_node!(GtReference, (to_parse_rules(Rule::name, "Hello"), &mut context)),
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
          definition_id: Unresolved,
          identifier: GtIdentifier(GtSpan(0, 5), "Hello"),
        )
        "#
        );
    }
}
