use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtGenericParameter {
    pub span: GtSpan,
    #[visit]
    pub identifier: GtIdentifier,
}

impl GtGenericParameter {
    pub fn parse(pair: Pair<'_, Rule>, _context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = pair.as_span().into();
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
