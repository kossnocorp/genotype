use crate::prelude::internal::*;

impl TsConvert<TsArray> for GtArray {
    fn convert(&self, context: &mut TsConvertContext) -> TsArray {
        TsArray {
            descriptor: self.descriptor.convert(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_node(Gt::array(Gt::primitive_boolean())),
            @"
        TsArray(
          descriptor: Primitive(Boolean),
        )
        "
        );
    }
}
