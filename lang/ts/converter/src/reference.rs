use genotype_lang_ts_tree::{definition::TSDefinition, reference::TSReference};
use genotype_parser::tree::reference::GTReference;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSReference> for GTReference {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSReference
    where
        HoistFn: Fn(TSDefinition),
    {
        TSReference(self.0.convert(resolve, hoist))
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSReference("Name".into()),
            GTReference("Name".into()).convert(&TSConvertResolve::new(), &|_| {}),
        );
    }
}
