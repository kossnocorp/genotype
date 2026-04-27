use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GtObject {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    #[visit]
    pub name: GtObjectName,
    #[visit]
    pub extensions: Vec<GtExtension>,
    #[visit]
    pub properties: Vec<GtProperty>,
}

impl GtObject {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = pair.as_span().into();
        let (doc, attributes) = context.take_annotation_or_default();

        let name = context.name_object(span)?;

        // It is an explicitly named object, so we need to add an anonymous parent so following
        // children don't get the object name.
        let named = matches!(name, GtObjectName::Named(_));
        if named {
            context.enter_parent(GtContextParent::Anonymous);
        }

        let mut object = GtObject {
            span,
            doc,
            attributes,
            name,
            extensions: vec![],
            properties: vec![],
        };

        if let Some(properties_pair) = pair.into_inner().next() {
            for pair in properties_pair.into_inner() {
                // Extract property rule
                let property_pair = pair
                    .into_inner()
                    .next()
                    .ok_or(GtParseError::UnexpectedEnd(span, GtNode::Object))?;

                match property_pair.as_rule() {
                    Rule::required_property | Rule::optional_property => {
                        object
                            .properties
                            .push(GtProperty::parse(property_pair, context)?);
                    }

                    Rule::extension_property => {
                        object
                            .extensions
                            .push(GtExtension::parse(property_pair, context)?);
                    }

                    rule => {
                        return Err(GtParseError::UnexpectedRule(
                            property_pair.as_span().into(),
                            GtNode::Object,
                            rule,
                        ));
                    }
                }
            }
        }

        if named {
            context.exit_parent(span, GtNode::Object)?;
        }

        Ok(object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::object, "{ hello: string }").unwrap();
        let mut context = GtContext {
            module_id: "module".into(),
            resolve: GtModuleResolve::new(),
            parents: vec![GtContextParent::Alias(GtIdentifier::new(
                (0, 5).into(),
                "Hello".into(),
            ))],
            claimed_names: Default::default(),
            annotation: None,
        };
        assert_ron_snapshot!(
            GtObject::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtObject(
          span: GtSpan(0, 17),
          doc: None,
          attributes: [],
          name: Named(GtIdentifier(GtSpan(0, 5), "Hello")),
          extensions: [],
          properties: [
            GtProperty(
              span: GtSpan(2, 15),
              doc: None,
              attributes: [],
              name: GtKey(GtSpan(2, 7), "hello"),
              descriptor: Primitive(GtPrimitive(
                span: GtSpan(9, 15),
                kind: String,
                doc: None,
                attributes: [],
              )),
              required: true,
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_parse_deps_base() {
        let source_code = r#"Order: {
                book: book/Book,
                user: ./misc/user/User
            }"#
        .to_owned();
        let parse = GtModule::parse("module".into(), &source_code).unwrap();
        assert_ron_snapshot!(
            parse.resolve.deps,
            @r#"
        [
          GtPath(
            span: GtSpan(31, 35),
            id: GtPathModuleId(
              span: GtSpan(31, 35),
              module_id: GtModuleId("module"),
            ),
            path: "book",
          ),
          GtPath(
            span: GtSpan(64, 75),
            id: GtPathModuleId(
              span: GtSpan(64, 75),
              module_id: GtModuleId("module"),
            ),
            path: "./misc/user",
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_parse_deps_normalize() {
        let source_code = r#"Order: {
                book: book/Book,
                user: ./misc/../misc/./user/User
            }"#
        .to_owned();
        let parse = GtModule::parse("module".into(), &source_code).unwrap();
        assert_ron_snapshot!(
            parse.resolve.deps,
            @r#"
        [
          GtPath(
            span: GtSpan(31, 35),
            id: GtPathModuleId(
              span: GtSpan(31, 35),
              module_id: GtModuleId("module"),
            ),
            path: "book",
          ),
          GtPath(
            span: GtSpan(64, 85),
            id: GtPathModuleId(
              span: GtSpan(64, 85),
              module_id: GtModuleId("module"),
            ),
            path: "./misc/user",
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_parse_name() {
        let mut pairs = GenotypeParser::parse(Rule::object, "{ hello: string }").unwrap();
        let mut context = GtContext {
            module_id: "module".into(),
            resolve: GtModuleResolve::new(),
            parents: vec![
                GtContextParent::Alias(GtIdentifier::new((0, 5).into(), "Hello".into())),
                GtContextParent::Anonymous,
            ],
            claimed_names: Default::default(),
            annotation: None,
        };
        assert_ron_snapshot!(
            GtObject::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GtObject(
          span: GtSpan(0, 17),
          doc: None,
          attributes: [],
          name: Alias(GtIdentifier(GtSpan(0, 17), "HelloObj"), Alias(GtIdentifier(GtSpan(0, 5), "Hello"))),
          extensions: [],
          properties: [
            GtProperty(
              span: GtSpan(2, 15),
              doc: None,
              attributes: [],
              name: GtKey(GtSpan(2, 7), "hello"),
              descriptor: Primitive(GtPrimitive(
                span: GtSpan(9, 15),
                kind: String,
                doc: None,
                attributes: [],
              )),
              required: true,
            ),
          ],
        )
        "#
        );
    }

    #[test]
    fn test_annotation() {
        let mut context = Gt::context();
        context.enter_parent(GtContextParent::Alias(Gt::identifier("Hello")));
        context.provide_annotation((
            Gt::some_doc("Hello, world!"),
            vec![Gt::attribute(
                "example",
                Gt::attribute_assignment(Gt::literal_string("value")),
            )],
        ));
        assert_ron_snapshot!(
            parse_node!(GtObject, (to_parse_rules(Rule::object, "{}"), &mut context)),
            @r#"
        GtObject(
          span: GtSpan(0, 2),
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
          name: Named(GtIdentifier(GtSpan(0, 0), "Hello")),
          extensions: [],
          properties: [],
        )
        "#
        );
    }
}
