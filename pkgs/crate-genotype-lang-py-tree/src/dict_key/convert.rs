use crate::prelude::internal::*;

impl PyConvert<PyDictKey> for GtRecordKey {
    fn convert(&self, context: &mut PyConvertContext) -> PyDictKey {
        match self {
            GtRecordKey::Reference(reference) => {
                let reference = reference.convert(context);
                if !context.is_generic_parameter(&reference.identifier) {
                    context.track_reference(&reference);
                }
                PyDictKey::Reference(reference)
            }
            GtRecordKey::Boolean(_) => PyDictKey::Boolean,
            GtRecordKey::String(_) => PyDictKey::String,
            GtRecordKey::Int8(_)
            | GtRecordKey::Int16(_)
            | GtRecordKey::Int32(_)
            | GtRecordKey::Int64(_)
            | GtRecordKey::Int128(_)
            | GtRecordKey::IntSize(_)
            | GtRecordKey::IntU8(_)
            | GtRecordKey::IntU16(_)
            | GtRecordKey::IntU32(_)
            | GtRecordKey::IntU64(_)
            | GtRecordKey::IntU128(_)
            | GtRecordKey::IntUSize(_) => PyDictKey::Int,
            GtRecordKey::Number(_) | GtRecordKey::Float32(_) | GtRecordKey::Float64(_) => {
                PyDictKey::Float
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_node(GtRecordKey::Reference(Gt::reference_anon("AddressId"))),
            @r#"
        Reference(PyReference(
          identifier: PyIdentifier("AddressId"),
          arguments: [],
          forward: true,
        ))
        "#
        );

        assert_ron_snapshot!(
            convert_node(Gt::record_key_string()),
            @"String"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_boolean()),
            @"Boolean"
        );

        assert_ron_snapshot!(
            convert_node(Gt::record_key_i32()),
            @"Int"
        );

        assert_ron_snapshot!(
            convert_node(Gt::record_key_f64()),
            @"Float"
        );
    }
}
