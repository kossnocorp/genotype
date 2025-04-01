use genotype_lang_ts_tree::TSRecordKey;
use genotype_parser::GTRecordKey;

use crate::{context::TSConvertContext, convert::TSConvert};

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
            GTRecordKey::Boolean(_) => TSRecordKey::Boolean,

            GTRecordKey::Int64(_)
            | GTRecordKey::Int128(_)
            | GTRecordKey::IntSize(_)
            | GTRecordKey::IntU64(_)
            | GTRecordKey::IntU128(_)
            | GTRecordKey::IntUSize(_) => {
                // [TODO] Return an error instead of panicking. It is not
                // straightforward because it will require chaning `TSConvert`
                // to return `Result`.
                // See: https://github.com/kossnocorp/genotype/issues/8
                panic!("TypeScript records don't support BigInt as a key")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSRecordKey::String,
            GTRecordKey::String((0, 0).into()).convert(&mut Default::default()),
        );
        assert_eq!(
            TSRecordKey::Number,
            GTRecordKey::Int32((0, 0).into()).convert(&mut Default::default()),
        );
        assert_eq!(
            TSRecordKey::Number,
            GTRecordKey::Float64((0, 0).into()).convert(&mut Default::default()),
        );
        assert_eq!(
            TSRecordKey::Boolean,
            GTRecordKey::Boolean((0, 0).into()).convert(&mut Default::default()),
        );
    }
}
