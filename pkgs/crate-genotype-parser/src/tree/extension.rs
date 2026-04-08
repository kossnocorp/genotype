use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GtExtension {
    pub span: GtSpan,
    #[visit]
    pub reference: GtReference,
}

impl GtExtension {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span = pair.as_span().into();

        match pair.into_inner().next() {
            Some(pair) => Ok(GtExtension {
                span,
                reference: GtReference::parse(pair, context)?,
            }),

            None => Err(GtParseError::Internal(span, GtNode::Extension)),
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
        let mut context = GtContext::new("module".into());
        assert_eq!(
            GtExtension::parse(pairs.next().unwrap(), &mut context).unwrap(),
            GtExtension {
                span: (0, 8).into(),
                reference: GtReference {
                    span: (3, 8).into(),
                    doc: None,
                    attributes: vec![],
                    id: GtReferenceId("module".into(), (3, 8).into()),
                    identifier: GtIdentifier::new((3, 8).into(), "Hello".into()),
                },
            }
        );
    }

    #[test]
    fn test_error() {
        let mut pairs = GenotypeParser::parse(Rule::literal_boolean, "false").unwrap();
        let mut context = GtContext::new("module".into());
        assert_eq!(
            GtExtension::parse(pairs.next().unwrap(), &mut context).unwrap_err(),
            GtParseError::Internal((0, 5).into(), GtNode::Extension)
        );
    }
}
