use genotype_lang_py_tree::PYDictKey;
use genotype_parser::GTRecordKey;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYDictKey> for GTRecordKey {
    fn convert(&self, _context: &mut PYConvertContext) -> PYDictKey {
        match self {
            GTRecordKey::String(_) => PYDictKey::String,
            GTRecordKey::Int(_) => PYDictKey::Int,
            GTRecordKey::Float(_) => PYDictKey::Float,
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
            GTRecordKey::Int((0, 0).into()).convert(&mut PYConvertContext::default()),
        );
        assert_eq!(
            PYDictKey::Float,
            GTRecordKey::Float((0, 0).into()).convert(&mut PYConvertContext::default()),
        );
        assert_eq!(
            PYDictKey::Boolean,
            GTRecordKey::Boolean((0, 0).into()).convert(&mut PYConvertContext::default()),
        );
    }
}
