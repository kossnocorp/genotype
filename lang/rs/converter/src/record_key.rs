use genotype_lang_rs_tree::RSDictKey;
use genotype_parser::GTRecordKey;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSDictKey> for GTRecordKey {
    fn convert(&self, _context: &mut RSConvertContext) -> RSDictKey {
        match self {
            GTRecordKey::String(_) => RSDictKey::String,
            GTRecordKey::Int(_) => RSDictKey::Int,
            GTRecordKey::Float(_) => RSDictKey::Float,
            GTRecordKey::Boolean(_) => RSDictKey::Boolean,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::context::RSConvertContext;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            RSDictKey::String,
            GTRecordKey::String((0, 0).into()).convert(&mut RSConvertContext::default()),
        );
        assert_eq!(
            RSDictKey::Int,
            GTRecordKey::Int((0, 0).into()).convert(&mut RSConvertContext::default()),
        );
        assert_eq!(
            RSDictKey::Float,
            GTRecordKey::Float((0, 0).into()).convert(&mut RSConvertContext::default()),
        );
        assert_eq!(
            RSDictKey::Boolean,
            GTRecordKey::Boolean((0, 0).into()).convert(&mut RSConvertContext::default()),
        );
    }
}
