use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtRecord {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    #[visit]
    pub key: GtRecordKey,
    #[visit]
    pub descriptor: GtDescriptor,
}

impl GtRecord {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = pair.as_span().into();
        let annotation = context.take_annotation_or_default();

        let mut inner = pair.into_inner();
        let pair = inner.next().ok_or(GtParseError::UnexpectedEnd(
            span,
            GtNode::Record,
            "record inner",
        ))?;

        let record = parse(inner, pair, context, ParseState::Key(span, annotation))?;

        Ok(record)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    context: &mut GtContext,
    state: ParseState,
) -> GtNodeParseResult<GtRecord> {
    match state {
        ParseState::Key(span, annotation) => {
            let key = GtRecordKey::parse(pair, context)?;

            match inner.next() {
                Some(pair) => parse(
                    inner,
                    pair,
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
            let descriptor = GtDescriptor::parse(pair, context)?;
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

enum ParseState {
    Key(GtSpan, GtContextAnnotation),
    Descriptor(GtSpan, GtContextAnnotation, GtRecordKey),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse_default() {
        assert_ron_snapshot!(
            parse_node!(GtRecord, to_parse_args(Rule::record, "{ []: string }")),
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
            parse_node!(GtRecord, to_parse_args(Rule::record, "{ [int]: string }")),
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
                to_parse_rules(Rule::record, "{ [Path]: string }"),
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
            parse_node!(GtRecord, (to_parse_rules(Rule::record, "{ []: string }"), &mut context)),
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
