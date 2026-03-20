use crate::prelude::internal::*;

impl TSConvert<TSRecordKey> for GTRecordKey {
    fn convert(&self, _context: &mut TSConvertContext) -> TSRecordKey {
        match self {
            GTRecordKey::String(_) => TSRecordKey::String,
            GTRecordKey::Number(_)
            | GTRecordKey::Int8(_)
            | GTRecordKey::Int16(_)
            | GTRecordKey::Int32(_)
            | GTRecordKey::IntU8(_)
            | GTRecordKey::IntU16(_)
            | GTRecordKey::IntU32(_)
            | GTRecordKey::Float32(_)
            | GTRecordKey::Float64(_) => TSRecordKey::Number,

            GTRecordKey::Int64(_)
            | GTRecordKey::Int128(_)
            | GTRecordKey::IntSize(_)
            | GTRecordKey::IntU64(_)
            | GTRecordKey::IntU128(_)
            | GTRecordKey::IntUSize(_) => {
                // [TODO] Return an error instead of panicking. It is not
                // straightforward because it will require changing `TSConvert`
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
            GTRecordKey::String((0, 0).into()).convert(&mut Default::default()),
            @"String"
        );
        assert_ron_snapshot!(
            GTRecordKey::Int32((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
        assert_ron_snapshot!(
            GTRecordKey::Float64((0, 0).into()).convert(&mut Default::default()),
            @"Number"
        );
    }
}
