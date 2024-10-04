use genotype_lang_ts_tree::{definition::TSDefinition, path::TSPath};
use genotype_parser::tree::path::GTPath;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSPath> for GTPath {
    fn convert<HoistFn>(&self, _resolve: &TSConvertResolve, _hoist: &HoistFn) -> TSPath
    where
        HoistFn: Fn(TSDefinition),
    {
        TSPath(self.as_str().to_owned())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSPath("./path/to/module".into()),
            GTPath::new("./path/to/module".into()).convert(&TSConvertResolve::new(), &|_| {}),
        );
    }
}
