use crate::prelude::internal::*;

impl GtParse for GtGenericParameter {
    fn parse(token: GtToken<'_>, _context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = token.span;
        let identifier = GtIdentifier::new(span, token.as_str().into());
        Ok(Self { span, identifier })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_ron_snapshot!(
            parse_node!(GtGenericParameter, to_parse_args(GtTokenKind::Reference, "Payload")),
            @r#"
        GtGenericParameter(
          span: GtSpan(0, 7),
          identifier: GtIdentifier(GtSpan(0, 7), "Payload"),
        )
        "#
        );
    }
}
