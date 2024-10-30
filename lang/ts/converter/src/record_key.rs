use genotype_lang_ts_tree::{definition::TSDefinition, TSRecordKey};
use genotype_parser::GTRecordKey;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSRecordKey> for GTRecordKey {
    fn convert<HoistFn>(&self, _resolve: &TSConvertResolve, _hoist: &HoistFn) -> TSRecordKey
    where
        HoistFn: Fn(TSDefinition),
    {
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
            GTRecordKey::String((0, 0).into()).convert(&TSConvertResolve::new(), &|_| {}),
        );
        assert_eq!(
            TSRecordKey::Number,
            GTRecordKey::Int((0, 0).into()).convert(&TSConvertResolve::new(), &|_| {}),
        );
        assert_eq!(
            TSRecordKey::Number,
            GTRecordKey::Float((0, 0).into()).convert(&TSConvertResolve::new(), &|_| {}),
        );
        assert_eq!(
            TSRecordKey::Boolean,
            GTRecordKey::Boolean((0, 0).into()).convert(&TSConvertResolve::new(), &|_| {}),
        );
    }
}
