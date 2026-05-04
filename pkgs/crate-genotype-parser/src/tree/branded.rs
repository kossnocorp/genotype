use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtBranded {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    pub id: GtDefinitionId,
    #[visit]
    pub name: GtIdentifier,
    #[visit]
    pub primitive: GtPrimitive,
}

impl GtBranded {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = pair.as_span().into();
        let (doc, attributes) = context.take_annotation_or_default();
        let pair = pair
            .into_inner()
            .next()
            .ok_or(GtParseError::InternalLegacy(span, GtNode::Array))?;
        let primitive = GtPrimitive::parse(pair, context)?;
        let name = context.get_name(&span, &primitive.to_string());
        let id = context.get_definition_id(&name);

        Ok(GtBranded {
            span,
            doc,
            attributes,
            id,
            name,
            primitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::branded, "@int").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtBranded::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtBranded(
          span: GtSpan(0, 4),
          doc: None,
          attributes: [],
          id: GtDefinitionId(GtModuleId("module"), "I64"),
          name: GtIdentifier(GtSpan(0, 4), "I64"),
          primitive: GtPrimitive(
            span: GtSpan(1, 4),
            kind: Int64,
            doc: None,
            attributes: [],
          ),
        )
        "#
        );
    }

    #[test]
    fn test_alias() {
        let mut pairs = GenotypeParser::parse(Rule::branded, "@int").unwrap();
        let mut context = GtContext::new("module".into());
        context.enter_named_parent(GtContextParent::Alias(GtIdentifier::new(
            GtSpan(0, 3),
            "Id".into(),
        )));
        assert_ron_snapshot!(
            GtBranded::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtBranded(
          span: GtSpan(0, 4),
          doc: None,
          attributes: [],
          id: GtDefinitionId(GtModuleId("module"), "Id"),
          name: GtIdentifier(GtSpan(0, 3), "Id"),
          primitive: GtPrimitive(
            span: GtSpan(1, 4),
            kind: Int64,
            doc: None,
            attributes: [],
          ),
        )
        "#
        );
    }

    #[test]
    fn test_anonymous() {
        let mut pairs = GenotypeParser::parse(Rule::branded, "@int").unwrap();
        let mut context = GtContext::new("module".into());
        context.enter_named_parent(GtContextParent::Alias(GtIdentifier::new(
            GtSpan(0, 3),
            "Id".into(),
        )));
        context.enter_named_parent(GtContextParent::Anonymous);
        assert_ron_snapshot!(
            GtBranded::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtBranded(
          span: GtSpan(0, 4),
          doc: None,
          attributes: [],
          id: GtDefinitionId(GtModuleId("module"), "IdI64"),
          name: GtIdentifier(GtSpan(0, 4), "IdI64"),
          primitive: GtPrimitive(
            span: GtSpan(1, 4),
            kind: Int64,
            doc: None,
            attributes: [],
          ),
        )
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
            parse_node!(GtBranded, (to_parse_rules(Rule::branded, "@int"), &mut context)),
            @r#"
        GtBranded(
          span: GtSpan(0, 4),
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
          id: GtDefinitionId(GtModuleId("module"), "I64"),
          name: GtIdentifier(GtSpan(0, 4), "I64"),
          primitive: GtPrimitive(
            span: GtSpan(1, 4),
            kind: Int64,
            doc: None,
            attributes: [],
          ),
        )
        "#
        );
    }
}
