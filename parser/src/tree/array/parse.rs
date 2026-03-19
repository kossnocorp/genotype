use crate::prelude::internal::*;

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
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::array, "[string]").unwrap();
        assert_ron_snapshot!(
            GTArray::parse(pairs.next().unwrap(), &mut GTContext::new("module".into())).unwrap(),
            @"
        GTArray(
          span: GTSpan(0, 8),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(1, 7),
            kind: String,
            doc: None,
            attributes: [],
          )),
        )
        "
        );
    }

    #[test]
    fn test_error() {
        let mut pairs = GenotypeParser::parse(Rule::literal_boolean, "false").unwrap();
        assert_equal!(
            GTArray::parse(pairs.next().unwrap(), &mut GTContext::new("module".into()))
                .unwrap_err(),
            GTParseError::Internal((0, 5).into(), GTNode::Array)
        );
    }
}
