use crate::prelude::internal::*;

impl GTTuple {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTParseError> {
        let span = pair.as_span().into();
        let mut tuple = GTTuple {
            span,
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
        let mut pairs = GenotypeParser::parse(Rule::tuple, "(string, int)").unwrap();
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTTuple::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @"
        GTTuple(
          span: GTSpan(0, 13),
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
