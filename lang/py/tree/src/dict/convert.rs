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
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_to_py(
                GtFactory::record(GtFactory::record_key_string(), GtFactory::primitive_string())
            ),
            @"
        PYDict(
          key: String,
          descriptor: Primitive(String),
        )
        "
        );
    }
}
