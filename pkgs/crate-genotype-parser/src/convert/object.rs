use crate::prelude::internal::*;

impl GtParse for GtObject {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = token.span;
        let (doc, attributes) = context.take_annotation_or_default();

        let name = context.name_object(span)?;

        // It is an explicitly named object, so we need to add an anonymous parent so following
        // children don't get the object name.
        let named = matches!(name, GtObjectName::Named(_));
        if named {
            context.enter_named_parent(GtContextParent::Anonymous);
        }

        let mut object = GtObject {
            span,
            doc,
            attributes,
            name,
            extensions: vec![],
            properties: vec![],
        };

        if let Some(properties_token) = token.into_semantic_children().next() {
            for token in properties_token.into_semantic_children() {
                // Extract property kind
                let property_token =
                    token
                        .into_semantic_children()
                        .next()
                        .ok_or(GtParseError::UnexpectedEnd(
                            span,
                            GtNode::Object,
                            "property inner",
                        ))?;

                match property_token.kind {
                    GtTokenKind::RequiredProperty | GtTokenKind::OptionalProperty => {
                        object
                            .properties
                            .push(GtProperty::parse(property_token, context)?);
                    }

                    GtTokenKind::ExtensionProperty => {
                        object
                            .extensions
                            .push(GtExtension::parse(property_token, context)?);
                    }

                    kind => {
                        return Err(GtParseError::UnexpectedToken(
                            (property_token.span.0, property_token.span.1).into(),
                            GtNode::Object,
                            kind,
                            "expected object property or extension",
                        ));
                    }
                }
            }
        }

        if named {
            context.exit_named_parent(span, GtNode::Object)?;
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
        let mut tokens = GtParser::parse(GtTokenKind::Object, "{ hello: string }").unwrap();
        let mut context = GtContext {
            module_id: "module".into(),
            named_parents: vec![GtContextParent::Alias(GtIdentifier::new(
                (0, 5).into(),
                "Hello".into(),
            ))],
            ..Default::default()
        };
        assert_ron_snapshot!(
            GtObject::parse(tokens.next().unwrap(), &mut context).unwrap(),
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
          GtModuleSource(
            span: GtSpan(31, 40),
            path: GtPath(
              span: GtSpan(31, 36),
              id: GtPathModuleId(
                span: GtSpan(31, 36),
                module_id: GtModuleId("module"),
              ),
              path: "book",
            ),
          ),
          GtModuleSource(
            span: GtSpan(64, 80),
            path: GtPath(
              span: GtSpan(64, 76),
              id: GtPathModuleId(
                span: GtSpan(64, 76),
                module_id: GtModuleId("module"),
              ),
              path: "./misc/user",
            ),
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
          GtModuleSource(
            span: GtSpan(31, 40),
            path: GtPath(
              span: GtSpan(31, 36),
              id: GtPathModuleId(
                span: GtSpan(31, 36),
                module_id: GtModuleId("module"),
              ),
              path: "book",
            ),
          ),
          GtModuleSource(
            span: GtSpan(64, 90),
            path: GtPath(
              span: GtSpan(64, 86),
              id: GtPathModuleId(
                span: GtSpan(64, 86),
                module_id: GtModuleId("module"),
              ),
              path: "./misc/user",
            ),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_parse_name() {
        let mut tokens = GtParser::parse(GtTokenKind::Object, "{ hello: string }").unwrap();
        let mut context = GtContext {
            module_id: "module".into(),
            named_parents: vec![
                GtContextParent::Alias(GtIdentifier::new((0, 5).into(), "Hello".into())),
                GtContextParent::Anonymous,
            ],
            ..Default::default()
        };
        assert_ron_snapshot!(
            GtObject::parse(tokens.next().unwrap(), &mut context).unwrap(),
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
        context.enter_named_parent(GtContextParent::Alias(Gt::identifier("Hello")));
        context.provide_annotation((
            Gt::some_doc("Hello, world!"),
            vec![Gt::attribute(
                "example",
                Gt::attribute_assignment(Gt::literal_string("value")),
            )],
        ));
        assert_ron_snapshot!(
            parse_node!(GtObject, (to_parse_tokens(GtTokenKind::Object, "{}"), &mut context)),
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
