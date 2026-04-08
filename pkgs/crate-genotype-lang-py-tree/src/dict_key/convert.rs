use crate::prelude::internal::*;

impl PyConvert<PyDictKey> for GtRecordKey {
    fn convert(&self, _context: &mut PyConvertContext) -> PyDictKey {
        match self {
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

    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_node(Gt::record_key_string()),
            @"String"
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
