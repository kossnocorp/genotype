use genotype_lang_py_tree::PYDictKey;
use genotype_parser::GTRecordKey;

use crate::{context::PYConvertContext, convert::PYConvert};

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
    use pretty_assertions::assert_eq;

    use crate::context::PYConvertContext;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            PYDictKey::String,
            GTRecordKey::String((0, 0).into()).convert(&mut PYConvertContext::default()),
        );
        assert_eq!(
            PYDictKey::Int,
            GTRecordKey::Int32((0, 0).into()).convert(&mut PYConvertContext::default()),
        );
        assert_eq!(
            PYDictKey::Float,
            GTRecordKey::Float64((0, 0).into()).convert(&mut PYConvertContext::default()),
        );
        assert_eq!(
            PYDictKey::Boolean,
            GTRecordKey::Boolean((0, 0).into()).convert(&mut PYConvertContext::default()),
        );
    }
}
