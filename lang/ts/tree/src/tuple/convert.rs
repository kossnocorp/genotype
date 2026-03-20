use crate::prelude::internal::*;

impl TSConvert<TSTuple> for GTTuple {
    fn convert(&self, context: &mut TSConvertContext) -> TSTuple {
        TSTuple {
            descriptors: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(context))
                .collect(),
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
            convert_to_ts(GtFactory::tuple(vec![
                GtFactory::primitive_boolean().into(),
                GtFactory::primitive_string().into(),
            ])),
            @"
        TSTuple(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
        )
        "
        );
    }
}
