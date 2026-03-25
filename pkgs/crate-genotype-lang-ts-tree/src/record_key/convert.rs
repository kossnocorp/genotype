use crate::prelude::internal::*;

impl TsConvert<TsRecordKey> for GtRecordKey {
    fn convert(&self, _context: &mut TsConvertContext) -> TsRecordKey {
        match self {
            GtRecordKey::String(_) => TsRecordKey::String,
            GtRecordKey::Number(_)
            | GtRecordKey::Int8(_)
            | GtRecordKey::Int16(_)
            | GtRecordKey::Int32(_)
            | GtRecordKey::IntU8(_)
            | GtRecordKey::IntU16(_)
            | GtRecordKey::IntU32(_)
            | GtRecordKey::Float32(_)
            | GtRecordKey::Float64(_) => TsRecordKey::Number,

            GtRecordKey::Int64(_)
            | GtRecordKey::Int128(_)
            | GtRecordKey::IntSize(_)
            | GtRecordKey::IntU64(_)
            | GtRecordKey::IntU128(_)
            | GtRecordKey::IntUSize(_) => {
                // [TODO] Return an error instead of panicking. It is not
                // straightforward because it will require changing `TsConvert`
                // to return `Result`.
                // See: https://github.com/kossnocorp/genotype/issues/8
                panic!("TypeScript records don't support BigInt as a key")
            }
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
            GtRecordKey::String((0, 0).into()).convert(&mut Default::default()),
            @"String"
        );
        assert_ron_snapshot!(
            GtRecordKey::Int32((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
        assert_ron_snapshot!(
            GtRecordKey::Float64((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
    }
}
