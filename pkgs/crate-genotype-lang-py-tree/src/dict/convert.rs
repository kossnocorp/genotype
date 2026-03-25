use crate::prelude::internal::*;

impl PyConvert<PyDict> for GtRecord {
    fn convert(&self, context: &mut PyConvertContext) -> PyDict {
        PyDict {
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
            convert_node(
                Gt::record(Gt::record_key_string(), Gt::primitive_string())
            ),
            @"
        PyDict(
          key: String,
          descriptor: Primitive(String),
        )
        "
        );
    }
}
