use crate::prelude::internal::*;

impl TSConvert<TSArray> for GTArray {
    fn convert(&self, context: &mut TSConvertContext) -> TSArray {
        TSArray {
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
        TSArray(
          descriptor: Primitive(Boolean),
        )
        "
        );
    }
}
