use crate::prelude::internal::*;

impl GtParse for GtAttributeAssignment {
    fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = (pair.as_span().start(), pair.as_span().end()).into();

        let mut inner = pair.into_inner();
        let pair = inner.next().ok_or(GtParseError::UnexpectedEnd(
            span,
            GtNode::AttributeAssignment,
            "attribute assignment inner",
        ))?;

        Ok(GtAttributeAssignment {
            span,
            value: GtAttributeValue::parse(pair, context)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::internal::*;
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
