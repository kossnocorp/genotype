use pest::iterators::Pair;

use crate::*;

use super::GTAttributeAssignment;

impl GTAttributeAssignment {
    pub fn parse(pair: Pair<'_, Rule>) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner.next().ok_or_else(|| {
            GTParseError::UnexpectedEnd(span.clone(), GTNode::AttributeAssignment)
        })?;

        Ok(GTAttributeAssignment {
            span,
            value: GTAttributeValue::parse(pair)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::attribute_assignment, "= 42").unwrap();
        assert_eq!(
            GTAttributeAssignment::new((0, 4).into(), GTLiteral::Integer((2, 4).into(), 42).into()),
            GTAttributeAssignment::parse(pairs.next().unwrap()).unwrap(),
        );
    }
}
