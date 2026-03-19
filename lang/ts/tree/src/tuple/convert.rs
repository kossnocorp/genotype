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
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTTuple {
                span: (0, 0).into(),
                descriptors: vec![
                    GtFactory::primitive_boolean().into(),
                    GtFactory::primitive_string().into(),
                ]
            }
            .convert(&mut Default::default()),
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
