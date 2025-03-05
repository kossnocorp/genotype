use crate::diagnostic::error::GTParseError;
use parser::Rule;
use pest::iterators::{Pair, Pairs};
use tree::GTDescriptor;

use crate::*;

use super::GTRecord;

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
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_parse_default() {
        let mut pairs = GenotypeParser::parse(Rule::record, "{ []: string }").unwrap();
        let mut context = GTContext::new("module".into());
        assert_eq!(
            GTRecord::parse(pairs.next().unwrap(), &mut context).unwrap(),
            GTRecord {
                span: (0, 14).into(),
                key: GTRecordKey::String((2, 4).into()),
                descriptor: GTPrimitive::String((6, 12).into()).into(),
            }
        );
    }

    #[test]
    fn test_parse_typed() {
        let mut pairs = GenotypeParser::parse(Rule::record, "{ [int]: string }").unwrap();
        let mut context = GTContext::new("module".into());
        assert_eq!(
            GTRecord::parse(pairs.next().unwrap(), &mut context).unwrap(),
            GTRecord {
                span: (0, 17).into(),
                key: GTRecordKey::Int64((2, 7).into()),
                descriptor: GTPrimitive::String((9, 15).into()).into(),
            }
        );
    }
}
