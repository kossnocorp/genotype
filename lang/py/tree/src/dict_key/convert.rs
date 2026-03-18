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
            GTRecordKey::Boolean(_) => PYDictKey::Boolean,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTRecordKey::String((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"String"
        );
        assert_ron_snapshot!(
            GTRecordKey::Int32((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GTRecordKey::Float64((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Float"
        );
        assert_ron_snapshot!(
            GTRecordKey::Boolean((0, 0).into()).convert(&mut PYConvertContext::default()),
            @"Boolean"
        );
    }
}
