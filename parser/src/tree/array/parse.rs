use pest::iterators::Pair;

use crate::*;

use super::GTArray;

impl GTArray {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();
        let pair = pair
            .into_inner()
            .next()
            .ok_or_else(|| GTParseError::Internal(span.clone(), GTNode::Array))?;
        let descriptor = GTDescriptor::parse(pair, context)?;
        Ok(GTArray { span, descriptor })
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::array, "[string]").unwrap();
        assert_eq!(
            GTArray::parse(pairs.next().unwrap(), &mut GTContext::new("module".into())).unwrap(),
            GTArray {
                span: (0, 8).into(),
                descriptor: GTDescriptor::Primitive(GTPrimitive::String((1, 7).into())),
            }
        );
    }

    #[test]
    fn test_error() {
        let mut pairs = GenotypeParser::parse(Rule::literal_boolean, "false").unwrap();
        assert_eq!(
            GTArray::parse(pairs.next().unwrap(), &mut GTContext::new("module".into()))
                .unwrap_err(),
            GTParseError::Internal((0, 5).into(), GTNode::Array)
        );
    }
}
