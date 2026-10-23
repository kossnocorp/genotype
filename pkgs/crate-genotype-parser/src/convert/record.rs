use crate::prelude::internal::*;

impl GtParse for GtRecord {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = token.span;
        let annotation = context.take_annotation_or_default();

        let mut inner = token.into_semantic_children();
        let token = inner.next().ok_or(GtParseError::UnexpectedEnd(
            span,
            GtNode::Record,
            "record inner",
        ))?;

        let record = parse(inner, token, context, ParseState::Key(span, annotation))?;

        Ok(record)
    }
}

enum ParseState {
    Key(GtSpan, GtContextAnnotation),
    Descriptor(GtSpan, GtContextAnnotation, GtRecordKey),
}

fn parse(
    mut inner: GtTokens<'_>,
    token: GtToken<'_>,
    context: &mut GtContext,
    state: ParseState,
) -> GtNodeParseResult<GtRecord> {
    match state {
        ParseState::Key(span, annotation) => {
            let key = GtRecordKey::parse(token, context)?;

            match inner.next() {
                Some(token) => parse(
                    inner,
                    token,
                    context,
                    ParseState::Descriptor(span, annotation, key),
                ),

                None => Err(GtParseError::UnexpectedEnd(
                    span,
                    GtNode::Record,
                    "continuation after record key",
                )),
            }
        }

        ParseState::Descriptor(span, annotation, key) => {
            let descriptor = GtDescriptor::parse(token, context)?;
            let (doc, attributes) = annotation;

            Ok(GtRecord {
                span,
                doc,
                attributes,
                key,
                descriptor,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse_default() {
        assert_ron_snapshot!(
            parse_node!(GtRecord, to_parse_args(GtTokenKind::Record, "{ []: string }")),
            @"
        GtRecord(
          span: GtSpan(0, 14),
          doc: None,
          attributes: [],
          key: String(GtSpan(2, 4)),
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(6, 12),
            kind: String,
            doc: None,
            attributes: [],
          )),
        )
        "
        );
    }

    #[test]
    fn test_parse_typed() {
        assert_ron_snapshot!(
            parse_node!(GtRecord, to_parse_args(GtTokenKind::Record, "{ [int]: string }")),
            @"
        GtRecord(
          span: GtSpan(0, 17),
          doc: None,
          attributes: [],
          key: Int64(GtSpan(2, 7)),
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(9, 15),
            kind: String,
            doc: None,
            attributes: [],
          )),
        )
        "
        );
    }

    #[test]
    fn test_parse_reference_key() {
        let mut context = GtContext::new("module".into());
        let record = parse_node!(
            GtRecord,
            (
                to_parse_tokens(GtTokenKind::Record, "{ [Path]: string }"),
                &mut context
            )
        );

        assert!(matches!(record.key, GtRecordKey::Reference(_)));
        assert_eq!(context.resolve.references[0].as_str(), "Path");
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
            parse_node!(GtRecord, (to_parse_tokens(GtTokenKind::Record, "{ []: string }"), &mut context)),
            @r#"
        GtRecord(
          span: GtSpan(0, 14),
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
          key: String(GtSpan(2, 4)),
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(6, 12),
            kind: String,
            doc: None,
            attributes: [],
          )),
        )
        "#
        );
    }
}
