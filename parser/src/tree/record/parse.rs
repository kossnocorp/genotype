use crate::prelude::internal::*;

impl GTRecord {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTParseError> {
        let span: GTSpan = pair.as_span().into();
        let annotation = context.take_annotation_or_default();

        let mut inner = pair.into_inner();
        let pair = inner
            .next()
            .ok_or_else(|| GTParseError::UnexpectedEnd(span.clone(), GTNode::Record))?;

        let record = parse(inner, pair, context, ParseState::Key(span, annotation))?;

        Ok(record)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    context: &mut GTContext,
    state: ParseState,
) -> GTNodeParseResult<GTRecord> {
    match state {
        ParseState::Key(span, annotation) => {
            let key = GTRecordKey::parse(pair)?;

            match inner.next() {
                Some(pair) => parse(
                    inner,
                    pair,
                    context,
                    ParseState::Descriptor(span, annotation, key),
                ),

                None => Err(GTParseError::UnexpectedEnd(span.clone(), GTNode::Record)),
            }
        }

        ParseState::Descriptor(span, annotation, key) => {
            let descriptor = GTDescriptor::parse(pair, context)?;
            let (doc, attributes) = annotation;

            Ok(GTRecord {
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
    Key(GTSpan, GTContextAnnotation),
    Descriptor(GTSpan, GTContextAnnotation, GTRecordKey),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse_default() {
        let mut pairs = GenotypeParser::parse(Rule::record, "{ []: string }").unwrap();
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTRecord::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @"
        GTRecord(
          span: GTSpan(0, 14),
          doc: None,
          attributes: [],
          key: String(GTSpan(2, 4)),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(6, 12),
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
        let mut pairs = GenotypeParser::parse(Rule::record, "{ [int]: string }").unwrap();
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTRecord::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @"
        GTRecord(
          span: GTSpan(0, 17),
          doc: None,
          attributes: [],
          key: Int64(GTSpan(2, 7)),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(9, 15),
            kind: String,
            doc: None,
            attributes: [],
          )),
        )
        "
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
            parse_node!(GTRecord, (to_parse_rules(Rule::record, "{ []: string }"), &mut context)),
            @r#"
        GTRecord(
          span: GTSpan(0, 14),
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
          key: String(GTSpan(2, 4)),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(6, 12),
            kind: String,
            doc: None,
            attributes: [],
          )),
        )
        "#
        );
    }
}
