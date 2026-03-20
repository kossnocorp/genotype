use crate::prelude::internal::*;

impl RSConvert<RSTuple> for GTTuple {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSTuple> {
        context.drop_definition_id();
        context.enter_parent(RSContextParent::Anonymous);

        let descriptors = self
            .descriptors
            .iter()
            .map(|descriptor| descriptor.convert(context))
            .collect::<Result<Vec<_>>>()?;
        let tuple = RSTuple { descriptors };

        context.exit_parent();
        Ok(tuple)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_to_rs(GtFactory::tuple(vec![
                GtFactory::primitive_boolean().into(),
                GtFactory::primitive_string().into(),
            ])),
            @"
        RSTuple(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
        )
        "
        );
    }
}
