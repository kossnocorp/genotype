use crate::prelude::internal::*;

impl GTRecord {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTParseError> {
        let span: GTSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner
            .next()
            .ok_or_else(|| GTParseError::UnexpectedEnd(span.clone(), GTNode::Record))?;

        let record = parse(inner, pair, context, ParseState::Key(span))?;

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
        ParseState::Key(span) => {
            let key = GTRecordKey::parse(pair)?;

            match inner.next() {
                Some(pair) => parse(inner, pair, context, ParseState::Descriptor(span, key)),

                None => Err(GTParseError::UnexpectedEnd(span.clone(), GTNode::Record)),
            }
        }

        ParseState::Descriptor(span, key) => {
            let descriptor = GTDescriptor::parse(pair, context)?;

            Ok(GTRecord {
                span,
                key,
                descriptor,
            })
        }
    }
}

enum ParseState {
    Key(GTSpan),
    Descriptor(GTSpan, GTRecordKey),
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
            @r"
        GTRecord(
          span: GTSpan(0, 14),
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
            @r"
        GTRecord(
          span: GTSpan(0, 17),
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
}
