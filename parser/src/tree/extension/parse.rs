use pest::iterators::Pair;

use crate::*;

use super::GTExtension;

impl GTExtension {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span = pair.as_span().into();

        match pair.into_inner().next() {
            Some(pair) => Ok(GTExtension {
                span,
                reference: GTReference::parse(pair, context),
            }),

            None => Err(GTParseError::Internal(span, GTNode::Extension)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::extension_property, "...Hello").unwrap();
        let mut context = GTContext::new("module".into());
        assert_eq!(
            GTExtension::parse(pairs.next().unwrap(), &mut context).unwrap(),
            GTExtension {
                span: (0, 8).into(),
                reference: GTReference(
                    (3, 8).into(),
                    GTReferenceDefinitionId::Unresolved,
                    GTIdentifier::new((3, 8).into(), "Hello".into()),
                ),
            }
        );
    }

    #[test]
    fn test_error() {
        let mut pairs = GenotypeParser::parse(Rule::literal_boolean, "false").unwrap();
        let mut context = GTContext::new("module".into());
        assert_eq!(
            GTExtension::parse(pairs.next().unwrap(), &mut context).unwrap_err(),
            GTParseError::Internal((0, 5).into(), GTNode::Extension)
        );
    }
}
