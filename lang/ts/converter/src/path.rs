use genotype_lang_ts_tree::{definition::TSDefinition, path::TSPath};
use genotype_parser::tree::path::GTPath;

use crate::convert::TSConvert;

impl TSConvert<TSPath> for GTPath {
    fn convert<HoistFn>(&self, _hoist: &HoistFn) -> TSPath
    where
        HoistFn: Fn(TSDefinition),
    {
        TSPath::Unresolved(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSPath::Unresolved("./path/to/module".into()),
            GTPath("./path/to/module".into()).convert(&|_| {}),
        );
    }
}
