use genotype_lang_ts_tree::TSRecordKey;
use genotype_parser::GTRecordKey;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSRecordKey> for GTRecordKey {
    fn convert(&self, _context: &mut TSConvertContext) -> TSRecordKey {
        match self {
            GTRecordKey::String(_) => TSRecordKey::String,
            GTRecordKey::Int(_) | GTRecordKey::Float(_) => TSRecordKey::Number,
            GTRecordKey::Boolean(_) => TSRecordKey::Boolean,
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
            GTRecordKey::Int((0, 0).into()).convert(&mut Default::default()),
        );
        assert_eq!(
            TSRecordKey::Number,
            GTRecordKey::Float((0, 0).into()).convert(&mut Default::default()),
        );
        assert_eq!(
            TSRecordKey::Boolean,
            GTRecordKey::Boolean((0, 0).into()).convert(&mut Default::default()),
        );
    }
}
