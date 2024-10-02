use genotype_lang_ts_tree::{definition::TSDefinition, doc::TSDoc};
use genotype_parser::tree::doc::GTDoc;

use crate::convert::TSConvert;

impl TSConvert<TSDoc> for GTDoc {
    fn convert<HoistFn>(&self, _hoist: &HoistFn) -> TSDoc
    where
        HoistFn: Fn(TSDefinition),
    {
        TSDoc(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSDoc("Hello, world!".into()),
            GTDoc("Hello, world!".into()).convert(&|_| {}),
        );
    }
}
