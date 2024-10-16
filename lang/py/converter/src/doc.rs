use genotype_lang_ts_tree::{definition::TSDefinition, doc::TSDoc};
use genotype_parser::tree::doc::GTDoc;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSDoc> for GTDoc {
    fn convert<HoistFn>(&self, _resolve: &TSConvertResolve, _hoist: &HoistFn) -> TSDoc
    where
        HoistFn: Fn(TSDefinition),
    {
        TSDoc(self.1.clone())
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
            GTDoc((0, 0).into(), "Hello, world!".into()).convert(&TSConvertResolve::new(), &|_| {}),
        );
    }
}
