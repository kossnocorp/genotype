use crate::diagnostic::error::GTNodeParseError;
use parser::Rule;
use pest::iterators::Pair;
use tree::GTDescriptor;

use crate::*;

use super::GTTuple;

impl GTTuple {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTNodeParseError> {
        let span = pair.as_span().into();
        let mut tuple = GTTuple {
            span,
            descriptors: vec![],
        };

        for pair in pair.into_inner() {
            let descriptor = GTDescriptor::parse(pair, context)?;
            tuple.descriptors.push(descriptor);
        }

        Ok(tuple)
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::tuple, "(string, int)").unwrap();
        let mut context = GTContext::new();
        assert_eq!(
            GTTuple::parse(pairs.next().unwrap(), &mut context).unwrap(),
            GTTuple {
                span: (0, 13).into(),
                descriptors: vec![
                    GTDescriptor::Primitive(GTPrimitive::String((1, 7).into())),
                    GTDescriptor::Primitive(GTPrimitive::Int((9, 12).into())),
                ],
            }
        );
    }
}
