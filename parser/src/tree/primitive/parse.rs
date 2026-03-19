use crate::prelude::internal::*;

impl GTPrimitive {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span = pair.as_span().into();
        let kind = GTPrimitiveKind::parse(pair, context)?;
        Ok(GTPrimitive {
            span,
            doc: None,
            attributes: vec![],
            kind,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        assert_ron_snapshot!(
            parse_node!(GTPrimitive, to_parse_args(Rule::primitive, "boolean")),
            @"
        GTPrimitive(
          span: GTSpan(0, 7),
          kind: Boolean,
          doc: None,
          attributes: [],
        )
        "
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitive, to_parse_args(Rule::primitive, "string")),
            @"
        GTPrimitive(
          span: GTSpan(0, 6),
          kind: String,
          doc: None,
          attributes: [],
        )
        "
        );

        assert_ron_snapshot!(
            parse_node!(GTPrimitive, to_parse_args(Rule::primitive, "number")),
            @"
        GTPrimitive(
          span: GTSpan(0, 6),
          kind: Number,
          doc: None,
          attributes: [],
        )
        "
        );
    }

    #[test]
    fn test_error() {
        assert_debug_snapshot!(
            parse_node_err!(GTPrimitive, to_parse_args(Rule::literal_boolean, "false")),
            @"
        Internal(
            GTSpan(
                0,
                5,
            ),
            Primitive,
        )
        "
        );
    }
}
