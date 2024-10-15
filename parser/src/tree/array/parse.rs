use pest::iterators::Pair;

use crate::{
    diagnostic::error::GTNodeParseError,
    parser::Rule,
    tree::{GTDescriptor, GTResolve},
    GTNode, GTNodeParseResult, GTSpan,
};

use super::GTArray;

impl GTArray {
    pub fn parse(pair: Pair<'_, Rule>, resolve: &mut GTResolve) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();
        let pair = pair
            .into_inner()
            .next()
            .ok_or_else(|| GTNodeParseError::Internal(span.clone(), GTNode::Array))?;
        let descriptor = GTDescriptor::parse(pair, resolve)?;
        Ok(GTArray { span, descriptor })
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use crate::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::array, "[string]").unwrap();
        assert_eq!(
            GTArray::parse(pairs.next().unwrap(), &mut GTResolve::new()).unwrap(),
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
            GTArray::parse(pairs.next().unwrap(), &mut GTResolve::new()).unwrap_err(),
            GTNodeParseError::Internal((0, 5).into(), GTNode::Array)
        );
    }
}
