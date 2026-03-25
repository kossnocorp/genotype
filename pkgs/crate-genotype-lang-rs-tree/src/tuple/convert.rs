use crate::prelude::internal::*;

impl RsConvert<RsTuple> for GtTuple {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsTuple> {
        context.drop_definition_id();
        context.enter_parent(RsContextParent::Anonymous);

        let descriptors = self
            .descriptors
            .iter()
            .map(|descriptor| descriptor.convert(context))
            .collect::<Result<Vec<_>>>()?;
        let tuple = RsTuple { descriptors };

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
            convert_node(Gt::tuple(vec![
                Gt::primitive_boolean().into(),
                Gt::primitive_string().into(),
            ])),
            @"
        RsTuple(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
        )
        "
        );
    }
}
