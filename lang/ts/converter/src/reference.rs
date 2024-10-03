use genotype_lang_ts_tree::{definition::TSDefinition, reference::TSReference, TSIdentifier};
use genotype_parser::tree::reference::GTReference;

use crate::convert::TSConvert;

impl TSConvert<TSReference> for GTReference {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSReference
    where
        HoistFn: Fn(TSDefinition),
    {
        TSReference(self.0.convert(hoist))
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
            GTReference("Name".into()).convert(&|_| {}),
        );
    }
}
