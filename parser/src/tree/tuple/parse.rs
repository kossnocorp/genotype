use crate::prelude::internal::*;

impl GTTuple {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTParseError> {
        let span = pair.as_span().into();
        let mut tuple = GTTuple {
            span,
            doc: None,
            attributes: vec![],
            descriptors: vec![],
        };

        for pair in pair.into_inner() {
            let descriptor = GTDescriptor::parse(pair, context)?;
            tuple.descriptors.push(descriptor);
        }

        Ok(tuple)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        assert_ron_snapshot!(
            parse_node!(GTTuple, to_parse_args(Rule::tuple, "(string, int)")),
            @"
        GTTuple(
          span: GTSpan(0, 13),
          doc: None,
          attributes: [],
          descriptors: [
            Primitive(GTPrimitive(
              span: GTSpan(1, 7),
              kind: String,
              doc: None,
              attributes: [],
            )),
            Primitive(GTPrimitive(
              span: GTSpan(9, 12),
              kind: Int64,
              doc: None,
              attributes: [],
            )),
          ],
        )
        "
        );
    }
}
