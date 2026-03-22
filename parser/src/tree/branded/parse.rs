use crate::prelude::internal::*;

impl GTBranded {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();
        let (doc, attributes) = context.take_annotation_or_default();
        let pair = pair
            .into_inner()
            .next()
            .ok_or_else(|| GTParseError::Internal(span.clone(), GTNode::Array))?;
        let primitive = GTPrimitive::parse(pair, context)?;
        let name = context.get_name(&span, &primitive.to_string());
        let id = context.get_definition_id(&name);

        Ok(GTBranded {
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
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTBranded::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GTBranded(
          span: GTSpan(0, 4),
          doc: None,
          attributes: [],
          id: GTDefinitionId(GTModuleId("module"), "I64"),
          name: GTIdentifier(GTSpan(0, 4), "I64"),
          primitive: GTPrimitive(
            span: GTSpan(1, 4),
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
        let mut context = GTContext::new("module".into());
        context.enter_parent(GTContextParent::Alias(GTIdentifier::new(
            GTSpan(0, 3),
            "Id".into(),
        )));
        assert_ron_snapshot!(
            GTBranded::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GTBranded(
          span: GTSpan(0, 4),
          doc: None,
          attributes: [],
          id: GTDefinitionId(GTModuleId("module"), "Id"),
          name: GTIdentifier(GTSpan(0, 3), "Id"),
          primitive: GTPrimitive(
            span: GTSpan(1, 4),
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
        let mut context = GTContext::new("module".into());
        context.enter_parent(GTContextParent::Alias(GTIdentifier::new(
            GTSpan(0, 3),
            "Id".into(),
        )));
        context.enter_parent(GTContextParent::Anonymous);
        assert_ron_snapshot!(
            GTBranded::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GTBranded(
          span: GTSpan(0, 4),
          doc: None,
          attributes: [],
          id: GTDefinitionId(GTModuleId("module"), "IdI64"),
          name: GTIdentifier(GTSpan(0, 4), "IdI64"),
          primitive: GTPrimitive(
            span: GTSpan(1, 4),
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
            parse_node!(GTBranded, (to_parse_rules(Rule::branded, "@int"), &mut context)),
            @r#"
        GTBranded(
          span: GTSpan(0, 4),
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
          id: GTDefinitionId(GTModuleId("module"), "I64"),
          name: GTIdentifier(GTSpan(0, 4), "I64"),
          primitive: GTPrimitive(
            span: GTSpan(1, 4),
            kind: Int64,
            doc: None,
            attributes: [],
          ),
        )
        "#
        );
    }
}
