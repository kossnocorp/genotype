use pest::iterators::Pair;

use crate::*;

use super::GTAttributeDescriptor;

impl GTAttributeDescriptor {
    pub fn parse(pair: Pair<'_, Rule>) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner.next().ok_or_else(|| {
            GTParseError::UnexpectedEnd(span.clone(), GTNode::AttributeDescriptor)
        })?;

        match pair.as_rule() {
            Rule::attribute_assignment => Ok(GTAttributeDescriptor::Assignment(
                GTAttributeAssignment::parse(pair)?,
            )),

            Rule::attribute_arguments => {
                let arguments = pair
                    .into_inner()
                    .map(GTAttributeValue::parse)
                    .collect::<Result<_, _>>()?;
                Ok(GTAttributeDescriptor::Arguments(arguments))
            }

            Rule::attribute_properties => {
                let properties = pair
                    .into_inner()
                    .map(GTAttributeProperty::parse)
                    .collect::<Result<_, _>>()?;
                Ok(GTAttributeDescriptor::Properties(properties))
            }

            _ => Err(GTParseError::UnknownRule(span, GTNode::AttributeDescriptor)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_assignment() {
        let mut pairs = GenotypeParser::parse(Rule::attribute_descriptor, "= 42").unwrap();
        assert_eq!(
            GTAttributeDescriptor::Assignment(GTAttributeAssignment::new(
                (0, 4).into(),
                GTLiteral::Integer((2, 4).into(), 42).into()
            )),
            GTAttributeDescriptor::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_arguments() {
        let mut pairs =
            GenotypeParser::parse(Rule::attribute_descriptor, r#"("hello", "world")"#).unwrap();
        assert_eq!(
            GTAttributeDescriptor::Arguments(vec![
                GTLiteral::String((1, 8).into(), "hello".into()).into(),
                GTLiteral::String((10, 17).into(), "world".into()).into()
            ]),
            GTAttributeDescriptor::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_properties() {
        let mut pairs = GenotypeParser::parse(
            Rule::attribute_descriptor,
            r#"(hello = "world", qwe = 123)"#,
        )
        .unwrap();
        assert_eq!(
            GTAttributeDescriptor::Properties(vec![
                GTAttributeProperty::new(
                    (1, 16).into(),
                    GTAttributeKey::new((1, 6).into(), "hello".into()),
                    GTLiteral::String((9, 16).into(), "world".into()).into(),
                ),
                GTAttributeProperty::new(
                    (18, 27).into(),
                    GTAttributeKey::new((18, 21).into(), "qwe".into()),
                    GTLiteral::Integer((24, 27).into(), 123).into(),
                ),
            ]),
            GTAttributeDescriptor::parse(pairs.next().unwrap()).unwrap(),
        );
    }
}
