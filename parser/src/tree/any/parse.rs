use crate::prelude::internal::*;

impl GTAny {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTParseError> {
        let span = pair.as_span().into();
        Ok(GTAny {
            span,
            doc: None,
            attributes: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test::*;
    use crate::*;

    #[test]
    fn test_from_pair() {
        assert_ron_snapshot!(
            parse_node!(GTAny, to_parse_args(Rule::any, "any")),
            @"
        GTAny(
          span: GTSpan(0, 3),
          doc: None,
          attributes: [],
        )
        "
        );
    }
}
