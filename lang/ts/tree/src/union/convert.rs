use crate::prelude::internal::*;

impl TSConvert<TSUnion> for GTUnion {
    fn convert(&self, context: &mut TSConvertContext) -> TSUnion {
        TSUnion {
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
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GtFactory::primitive_boolean().into(),
                    GtFactory::primitive_string().into(),
                ]
            }
            .convert(&mut Default::default()),
            @"
        TSUnion(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
        )
        "
        );
    }
}
