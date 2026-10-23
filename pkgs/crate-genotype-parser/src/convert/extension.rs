use crate::prelude::internal::*;

impl GtParse for GtExtension {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span = token.span;

        match token.into_semantic_children().next() {
            Some(token) if token.kind == GtTokenKind::InlineImport => {
                let inline_import = GtInlineImport::parse(token, context)?;
                let reference_span = inline_import.span;
                let reference = GtReference {
                    span: reference_span,
                    doc: inline_import.doc,
                    attributes: inline_import.attributes,
                    id: GtReferenceId(context.module_id.clone(), reference_span),
                    identifier: inline_import.name,
                    arguments: inline_import.arguments,
                };

                context
                    .resolve
                    .references
                    .insert(reference.identifier.clone());
                context.resolve_reference_identifier_as_generic_parameter(&reference.identifier);

                Ok(GtExtension { span, reference })
            }

            Some(token) => Ok(GtExtension {
                span,
                reference: GtReference::parse(token, context)?,
            }),

            None => Err(GtParseError::UnexpectedEnd(
                span,
                GtNode::Extension,
                "extension reference",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut tokens = GtParser::parse(GtTokenKind::ExtensionProperty, "...Hello").unwrap();
        let mut context = GtContext::new("module".into());
        assert_eq!(
            GtExtension::parse(tokens.next().unwrap(), &mut context).unwrap(),
            GtExtension {
                span: (0, 8).into(),
                reference: GtReference {
                    span: (3, 8).into(),
                    doc: None,
                    attributes: vec![],
                    id: GtReferenceId("module".into(), (3, 8).into()),
                    identifier: GtIdentifier::new((3, 8).into(), "Hello".into()),
                    arguments: vec![],
                },
            }
        );
    }

    #[test]
    fn test_error() {
        let mut tokens = GtParser::parse(GtTokenKind::LiteralBoolean, "false").unwrap();
        let mut context = GtContext::new("module".into());
        assert_eq!(
            GtExtension::parse(tokens.next().unwrap(), &mut context).unwrap_err(),
            GtParseError::UnexpectedEnd((0, 5).into(), GtNode::Extension, "extension reference")
        );
    }

    #[test]
    fn test_arguments() {
        assert_ron_snapshot!(
            parse_node!(GtExtension, to_parse_args(GtTokenKind::ExtensionProperty, "...Message<string>")),
            @r#"
        GtExtension(
          span: GtSpan(0, 18),
          reference: GtReference(
            span: GtSpan(3, 18),
            doc: None,
            attributes: [],
            id: GtReferenceId(GtModuleId("module"), GtSpan(3, 18)),
            identifier: GtIdentifier(GtSpan(3, 10), "Message"),
            arguments: [
              GtGenericArgument(
                span: GtSpan(11, 17),
                descriptor: Primitive(GtPrimitive(
                  span: GtSpan(11, 17),
                  kind: String,
                  doc: None,
                  attributes: [],
                )),
              ),
            ],
          ),
        )
        "#
        );
    }
}
