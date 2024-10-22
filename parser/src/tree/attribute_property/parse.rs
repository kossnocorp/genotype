use pest::iterators::{Pair, Pairs};

use crate::*;

use super::GTAttributeProperty;

impl GTAttributeProperty {
    pub fn parse(pair: Pair<'_, Rule>) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner
            .next()
            .ok_or_else(|| GTParseError::UnexpectedEnd(span.clone(), GTNode::AttributeProperty))?;

        let property = parse(inner, pair, ParseState::Name(span))?;

        Ok(property)
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
) -> GTNodeParseResult<GTAttributeProperty> {
    match state {
        ParseState::Name(span) => {
            let key = GTAttributeKey::parse(pair);

            match inner.next() {
                Some(pair) => parse(inner, pair, ParseState::Value(span, key)),

                None => Err(GTParseError::UnexpectedEnd(
                    span.clone(),
                    GTNode::AttributeProperty,
                )),
            }
        }

        ParseState::Value(span, name) => {
            let value = GTAttributeValue::parse(pair)?;

            Ok(GTAttributeProperty { span, name, value })
        }
    }
}

enum ParseState {
    Name(GTSpan),
    Value(GTSpan, GTAttributeKey),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs =
            GenotypeParser::parse(Rule::attribute_property, r#"hello = "world""#).unwrap();
        assert_eq!(
            GTAttributeProperty::new(
                (0, 15).into(),
                GTAttributeKey::new((0, 5).into(), "hello".into()),
                GTLiteral::String((8, 15).into(), "world".into()).into(),
            ),
            GTAttributeProperty::parse(pairs.next().unwrap()).unwrap(),
        );
    }
}
