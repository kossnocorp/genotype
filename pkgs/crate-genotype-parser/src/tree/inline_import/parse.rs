use crate::prelude::internal::*;

impl GTInlineImport {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTParseError> {
        let span = pair.as_span().into();
        let (path, (name_span, name)) = GTPath::split_parse(pair)?;
        let (doc, attributes) = context.take_annotation_or_default();

        context.resolve.deps.insert(path.clone());

        Ok(GTInlineImport {
            span,
            doc,
            attributes,
            path,
            name: GTIdentifier::new(name_span, name.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        assert_ron_snapshot!(
            parse_node!(GTInlineImport, to_parse_args(Rule::inline_import, "./path/to/module/Name")),
            @r#"
        GTInlineImport(
          span: GTSpan(0, 21),
          doc: None,
          attributes: [],
          name: GTIdentifier(GTSpan(17, 21), "Name"),
          path: GTPath(GTSpan(0, 16), Unresolved, "./path/to/module"),
        )
        "#
        );
    }

    #[test]
    fn test_parse_deps_base() {
        let source_code = NamedSource::new(
            "module.type",
            r#"Order: {
                book: book/Book,
                user: ./misc/user/User
            }"#
            .into(),
        );
        let parse = GTModule::parse("module".into(), source_code).unwrap();
        assert_ron_snapshot!(
            parse.resolve.deps,
            @r#"
        [
          GTPath(GTSpan(31, 35), Unresolved, "book"),
          GTPath(GTSpan(64, 75), Unresolved, "./misc/user"),
        ]
        "#
        );
    }

    #[test]
    fn test_parse_deps_normalize() {
        let source_code = NamedSource::new(
            "module.type",
            r#"Order: {
                book: book/Book,
                user: ./misc/../misc/./user/User
            }"#
            .into(),
        );
        let parse = GTModule::parse("module".into(), source_code).unwrap();
        assert_ron_snapshot!(
            parse.resolve.deps,
            @r#"
        [
          GTPath(GTSpan(31, 35), Unresolved, "book"),
          GTPath(GTSpan(64, 85), Unresolved, "./misc/user"),
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
            parse_node!(
                GTInlineImport,
                (to_parse_rules(Rule::inline_import, "./path/to/module/Name"), &mut context)
            ),
            @r#"
        GTInlineImport(
          span: GTSpan(0, 21),
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
          name: GTIdentifier(GTSpan(17, 21), "Name"),
          path: GTPath(GTSpan(0, 16), Unresolved, "./path/to/module"),
        )
        "#
        );
    }
}
