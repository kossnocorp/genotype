use crate::prelude::internal::*;

impl PYConvert<PYDictKey> for GTRecordKey {
    fn convert(&self, _context: &mut PYConvertContext) -> PYDictKey {
        match self {
            GTRecordKey::String(_) => PYDictKey::String,
            GTRecordKey::Int8(_)
            | GTRecordKey::Int16(_)
            | GTRecordKey::Int32(_)
            | GTRecordKey::Int64(_)
            | GTRecordKey::Int128(_)
            | GTRecordKey::IntSize(_)
            | GTRecordKey::IntU8(_)
            | GTRecordKey::IntU16(_)
            | GTRecordKey::IntU32(_)
            | GTRecordKey::IntU64(_)
            | GTRecordKey::IntU128(_)
            | GTRecordKey::IntUSize(_) => PYDictKey::Int,
            GTRecordKey::Number(_) | GTRecordKey::Float32(_) | GTRecordKey::Float64(_) => {
                PYDictKey::Float
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_to_py(GtFactory::record_key_string()),
            @"String"
        );
        assert_ron_snapshot!(
            convert_to_py(GtFactory::record_key_i32()),
            @"Int"
        );
        assert_ron_snapshot!(
            convert_to_py(GtFactory::record_key_f64()),
            @"Float"
        );
    }
}
