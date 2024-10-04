use genotype_lang_ts_tree::{definition::TSDefinition, identifier::TSIdentifier};
use genotype_parser::tree::identifier::GTIdentifier;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSIdentifier> for GTIdentifier {
    fn convert<HoistFn>(&self, _resolve: &TSConvertResolve, _hoist: &HoistFn) -> TSIdentifier
    where
        HoistFn: Fn(TSDefinition),
    {
        TSIdentifier(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSIdentifier("Foo".into()),
            GTIdentifier("Foo".into()).convert(&TSConvertResolve::new(), &|_| {}),
        );
    }
}
