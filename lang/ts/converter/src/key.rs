use genotype_lang_ts_tree::{definition::TSDefinition, key::TSKey};
use genotype_parser::tree::key::GTKey;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSKey> for GTKey {
    fn convert<HoistFn>(&self, _resolve: &TSConvertResolve, _hoist: &HoistFn) -> TSKey
    where
        HoistFn: Fn(TSDefinition),
    {
        TSKey(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSKey("foo".into()),
            GTKey("foo".into()).convert(&TSConvertResolve::new(), &|_| {}),
        );
    }
}
