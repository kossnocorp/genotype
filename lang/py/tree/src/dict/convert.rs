use crate::prelude::internal::*;

impl PYConvert<PYDict> for GTRecord {
    fn convert(&self, context: &mut PYConvertContext) -> PYDict {
        PYDict {
            key: self.key.convert(context),
            descriptor: self.descriptor.convert(context),
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTRecord {
                span: (0, 0).into(),
                key: GTRecordKey::String((0, 0).into()),
                descriptor: GtFactory::primitive_string().into(),
            }
            .convert(&mut PYConvertContext::default()),
            @"
        PYDict(
          key: String,
          descriptor: Primitive(String),
        )
        "
        );
    }
}
