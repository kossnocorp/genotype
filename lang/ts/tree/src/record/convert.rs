use crate::prelude::internal::*;

impl TSConvert<TSRecord> for GTRecord {
    fn convert(&self, context: &mut TSConvertContext) -> TSRecord {
        TSRecord {
            key: self.key.convert(context),
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
            convert_to_ts(
                GtFactory::record(GtFactory::record_key_string(), GtFactory::primitive_string())
            ),
            @"
        TSRecord(
          key: String,
          descriptor: Primitive(String),
        )
        "
        );
    }
}
