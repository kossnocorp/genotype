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
    use super::*;
    use genotype_test::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTArray {
                span: (0, 0).into(),
                descriptor: GtFactory::primitive_boolean().into(),
            }
            .convert(&mut Default::default()),
            @"
        TSArray(
          descriptor: Primitive(Boolean),
        )
        "
        );
    }
}
