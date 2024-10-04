use genotype_lang_ts_tree::{definition::TSDefinition, identifier::TSIdentifier};
use genotype_parser::tree::identifier::GTIdentifier;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSIdentifier> for GTIdentifier {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, _hoist: &HoistFn) -> TSIdentifier
    where
        HoistFn: Fn(TSDefinition),
    {
        TSIdentifier(resolve.identifiers.get(&self).unwrap_or(&self).0.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_base() {
        assert_eq!(
            TSIdentifier("Foo".into()),
            GTIdentifier("Foo".into()).convert(&TSConvertResolve::new(), &|_| {}),
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = TSConvertResolve::new();
        resolve.identifiers.insert("Foo".into(), "foo.Bar".into());
        assert_eq!(
            TSIdentifier("foo.Bar".into()),
            GTIdentifier("Foo".into()).convert(&resolve, &|_| {}),
        );
    }
}
