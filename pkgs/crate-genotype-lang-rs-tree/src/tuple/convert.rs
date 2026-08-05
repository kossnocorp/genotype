use crate::prelude::internal::*;

impl RsConvert<RsTuple> for GtTuple {
    fn convert(&self, context: &mut RsConvertContext) -> RsConvertResult<RsTuple> {
        context.drop_definition_id();

        let descriptors = self
            .descriptors
            .iter()
            .enumerate()
            .map(|(index, descriptor)| {
                context.enter_parent(RsContextParent::TupleElement(index));
                let descriptor = descriptor.convert(context);
                context.exit_parent();
                descriptor
            })
            .collect::<RsConvertResult<Vec<_>>>()?;
        let tuple = RsTuple { descriptors };

        Ok(tuple)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
