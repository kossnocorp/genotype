use pest::iterators::{Pair, Pairs};

use crate::*;

use super::GTAttribute;

impl GTAttribute {
    pub fn parse(pair: Pair<'_, Rule>) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner
            .next()
            .ok_or_else(|| GTParseError::UnexpectedEnd(span.clone(), GTNode::Attribute))?;

        parse(inner, pair, ParseState::Name(span))
    }
}

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
) -> GTNodeParseResult<GTAttribute> {
    match state {
        ParseState::Name(span) => {
            let name_span: GTSpan = pair.as_span().into();
            let name = GTAttributeName::new(name_span.clone(), pair.as_str().into());

            match inner.next() {
                Some(pair) => parse(inner, pair, ParseState::Descriptor(span, name)),

                None => Ok(GTAttribute {
                    span,
                    name,
                    descriptor: None,
                }),
            }
        }

        ParseState::Descriptor(span, name) => {
            let descriptor = match pair.as_rule() {
                Rule::attribute_descriptor => Some(GTAttributeDescriptor::parse(pair)?),
                _ => None,
            };

            Ok(GTAttribute {
                span,
                name,
                descriptor,
            })
        }
    }
}

enum ParseState {
    Name(GTSpan),
    Descriptor(GTSpan, GTAttributeName),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_simple() {
        let mut pairs = GenotypeParser::parse(Rule::attribute, "#[tag]").unwrap();
        assert_eq!(
            GTAttribute::new(
                (0, 6).into(),
                GTAttributeName::new((2, 5).into(), "tag".into()).into(),
                None
            ),
            GTAttribute::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_assignment() {
        let mut pairs = GenotypeParser::parse(Rule::attribute, "#[answer = 42]").unwrap();
        assert_eq!(
            GTAttribute::new(
                (0, 14).into(),
                GTAttributeName::new((2, 8).into(), "answer".into()).into(),
                Some(GTAttributeDescriptor::Assignment(
                    GTAttributeAssignment::new(
                        (9, 13).into(),
                        GTLiteral::Integer((11, 13).into(), 42).into()
                    )
                ))
            ),
            GTAttribute::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_arguments() {
        let mut pairs =
            GenotypeParser::parse(Rule::attribute, r#"#[say("hello", "world")]"#).unwrap();
        assert_eq!(
            GTAttribute::new(
                (0, 24).into(),
                GTAttributeName::new((2, 5).into(), "say".into()).into(),
                Some(GTAttributeDescriptor::Arguments(vec![
                    GTLiteral::String((6, 13).into(), "hello".into()).into(),
                    GTLiteral::String((15, 22).into(), "world".into()).into()
                ]))
            ),
            GTAttribute::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_properties() {
        let mut pairs =
            GenotypeParser::parse(Rule::attribute, r#"#[say(hello = "world", qwe = 123)]"#)
                .unwrap();
        assert_eq!(
            GTAttribute::new(
                (0, 34).into(),
                GTAttributeName::new((2, 5).into(), "say".into()).into(),
                Some(GTAttributeDescriptor::Properties(vec![
                    GTAttributeProperty::new(
                        (6, 21).into(),
                        GTAttributeKey::new((6, 11).into(), "hello".into()),
                        GTLiteral::String((14, 21).into(), "world".into()).into(),
                    ),
                    GTAttributeProperty::new(
                        (23, 32).into(),
                        GTAttributeKey::new((23, 26).into(), "qwe".into()),
                        GTLiteral::Integer((29, 32).into(), 123).into(),
                    ),
                ]))
            ),
            GTAttribute::parse(pairs.next().unwrap()).unwrap(),
        );
    }
}
