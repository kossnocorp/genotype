use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtAttributeAssignment {
    pub span: GtSpan,
    pub value: GtAttributeValue,
}

impl GtAttributeAssignment {
    pub fn new(span: GtSpan, value: GtAttributeValue) -> Self {
        Self { span, value }
    }
}

impl GtAttributeAssignment {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner
            .next()
            .ok_or_else(|| GtParseError::UnexpectedEnd(span, GtNode::AttributeAssignment))?;

        Ok(GtAttributeAssignment {
            span,
            value: GtAttributeValue::parse(pair, context)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use insta::assert_ron_snapshot;
    use pest::Parser;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::attribute_assignment, "= 42").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeAssignment::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @"
        GtAttributeAssignment(
          span: GtSpan(0, 4),
          value: Literal(GtLiteral(
            span: GtSpan(2, 4),
            doc: None,
            attributes: [],
            value: Integer(42),
          )),
        )
        "
        );
    }
}
