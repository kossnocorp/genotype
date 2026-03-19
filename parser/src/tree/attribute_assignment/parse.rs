use crate::prelude::internal::*;

impl GTAttributeAssignment {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner.next().ok_or_else(|| {
            GTParseError::UnexpectedEnd(span.clone(), GTNode::AttributeAssignment)
        })?;

        Ok(GTAttributeAssignment {
            span,
            value: GTAttributeValue::parse(pair, context)?,
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
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTAttributeAssignment::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @"
        GTAttributeAssignment(
          span: GTSpan(0, 4),
          value: Literal(GTLiteral(
            span: GTSpan(2, 4),
            doc: None,
            attributes: [],
            value: Integer(42),
          )),
        )
        "
        );
    }
}
