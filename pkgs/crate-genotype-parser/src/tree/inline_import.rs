use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtInlineImport {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    #[visit]
    pub name: GtIdentifier,
    #[visit]
    pub arguments: Vec<GtGenericArgument>,
    #[visit]
    pub path: GtPath,
}

impl GtInlineImport {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = pair.as_span().into();
        let (doc, attributes) = context.take_annotation_or_default();

        let mut inner = pair.into_inner();

        let pair = inner.next().ok_or_else(|| {
            GtParseError::UnexpectedEnd(span, GtNode::InlineImport, "inline import inner")
        })?;

        let path_span = (span.0, span.0).into();
        let (path, name, arguments) = Self::parse_path_with_name_and_arguments(
            inner,
            pair,
            context,
            ParseState::Path(path_span, String::new()),
        )?;

        context.resolve.deps.insert(path.clone());

        Ok(GtInlineImport {
            span,
            doc,
            attributes,
            path,
            name,
            arguments,
        })
    }

    fn parse_path_with_name_and_arguments(
        mut inner: Pairs<'_, Rule>,
        pair: Pair<'_, Rule>,
        context: &mut GtContext,
        state: ParseState,
    ) -> Result<(GtPath, GtIdentifier, Vec<GtGenericArgument>), GtParseError> {
        match state {
            ParseState::Path(path_span, path_str) => match pair.as_rule() {
                Rule::path_segment => {
                    let span = (path_span.0, pair.as_span().end()).into();
                    let path_str = path_str + pair.as_str();

                    match inner.next() {
                        Some(pair) => Self::parse_path_with_name_and_arguments(
                            inner,
                            pair,
                            context,
                            ParseState::Path(span, path_str),
                        ),

                        None => Err(GtParseError::UnexpectedEnd(
                            pair.as_span().into(),
                            GtNode::InlineImport,
                            "continuation after path",
                        )),
                    }
                }

                Rule::reference => {
                    let path = GtPath::parse(path_span, &context.module_id, &path_str)?;
                    Self::parse_path_with_name_and_arguments(
                        inner,
                        pair,
                        context,
                        ParseState::Reference(path),
                    )
                }

                _ => Err(GtParseError::InternalLegacy(
                    pair.as_span().into(),
                    GtNode::InlineImport,
                )),
            },

            ParseState::Reference(path) => {
                let (name, arguments) = GtReference::parse_name_with_arguments(pair, context)?;
                Ok((path, name, arguments))
            }
        }
    }
}

enum ParseState {
    Path(GtSpan, String),
    Reference(GtPath),
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
          arguments: [],
          path: GtPath(
            span: GtSpan(0, 17),
            id: GtPathModuleId(
              span: GtSpan(0, 17),
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
            span: GtSpan(31, 36),
            id: GtPathModuleId(
              span: GtSpan(31, 36),
              module_id: GtModuleId("module"),
            ),
            path: "book",
          ),
          GtPath(
            span: GtSpan(64, 76),
            id: GtPathModuleId(
              span: GtSpan(64, 76),
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
            span: GtSpan(31, 36),
            id: GtPathModuleId(
              span: GtSpan(31, 36),
              module_id: GtModuleId("module"),
            ),
            path: "book",
          ),
          GtPath(
            span: GtSpan(64, 86),
            id: GtPathModuleId(
              span: GtSpan(64, 86),
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
          arguments: [],
          path: GtPath(
            span: GtSpan(0, 17),
            id: GtPathModuleId(
              span: GtSpan(0, 17),
              module_id: GtModuleId("module"),
            ),
            path: "./path/to/module",
          ),
        )
        "#
        );
    }

    #[test]
    fn test_arguments() {
        assert_ron_snapshot!(
            parse_node!(GtInlineImport, to_parse_args(Rule::inline_import, "./path/to/module/Message<string>")),
            @r#"
        GtInlineImport(
          span: GtSpan(0, 32),
          doc: None,
          attributes: [],
          name: GtIdentifier(GtSpan(17, 24), "Message"),
          arguments: [
            GtGenericArgument(
              span: GtSpan(25, 31),
              descriptor: Primitive(GtPrimitive(
                span: GtSpan(25, 31),
                kind: String,
                doc: None,
                attributes: [],
              )),
            ),
          ],
          path: GtPath(
            span: GtSpan(0, 17),
            id: GtPathModuleId(
              span: GtSpan(0, 17),
              module_id: GtModuleId("module"),
            ),
            path: "./path/to/module",
          ),
        )
        "#
        );
    }
}
