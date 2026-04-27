use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GtInlineImport {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    #[visit]
    pub name: GtIdentifier,
    #[visit]
    pub path: GtPath,
}

impl GtInlineImport {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> Result<Self, GtParseError> {
        let span = pair.as_span().into();
        let (path, (name_span, name)) = GtPath::split_parse(pair, &context.module_id)?;
        let (doc, attributes) = context.take_annotation_or_default();

        context.resolve.deps.insert(path.clone());

        Ok(GtInlineImport {
            span,
            doc,
            attributes,
            path,
            name: GtIdentifier::new(name_span, name.into()),
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
            parse_node!(GtInlineImport, to_parse_args(Rule::inline_import, "./path/to/module/Name")),
            @r#"
        GtInlineImport(
          span: GtSpan(0, 21),
          doc: None,
          attributes: [],
          name: GtIdentifier(GtSpan(17, 21), "Name"),
          path: GtPath(
            span: GtSpan(0, 16),
            id: GtPathModuleId(
              span: GtSpan(0, 16),
              module_id: GtModuleId("module"),
            ),
            path: "./path/to/module",
          ),
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
                GtInlineImport,
                (to_parse_rules(Rule::inline_import, "./path/to/module/Name"), &mut context)
            ),
            @r#"
        GtInlineImport(
          span: GtSpan(0, 21),
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
          name: GtIdentifier(GtSpan(17, 21), "Name"),
          path: GtPath(
            span: GtSpan(0, 16),
            id: GtPathModuleId(
              span: GtSpan(0, 16),
              module_id: GtModuleId("module"),
            ),
            path: "./path/to/module",
          ),
        )
        "#
        );
    }
}
