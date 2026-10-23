use crate::prelude::internal::*;

impl GtParse for GtGenericParameter {
    fn parse(pair: Pair<'_, Rule>, _context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = (pair.as_span().start(), pair.as_span().end()).into();
        let identifier = GtIdentifier::new(span, pair.as_str().into());
        Ok(Self { span, identifier })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_ron_snapshot!(
            parse_node!(GtGenericParameter, to_parse_args(Rule::reference, "Payload")),
            @r#"
        GtGenericParameter(
          span: GtSpan(0, 7),
          identifier: GtIdentifier(GtSpan(0, 7), "Payload"),
        )
        "#
        );
    }
}
