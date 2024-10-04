use genotype_lang_ts_tree::{definition::TSDefinition, inline_import::TSInlineImport};
use genotype_parser::tree::inline_import::GTInlineImport;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSInlineImport> for GTInlineImport {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSInlineImport
    where
        HoistFn: Fn(TSDefinition),
    {
        TSInlineImport {
            path: self.path.convert(resolve, hoist),
            name: self.name.convert(resolve, hoist),
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
            GTInlineImport {
                path: "./path/to/module".into(),
                name: "Name".into(),
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSInlineImport {
                path: "./path/to/module".into(),
                name: "Name".into(),
            }
        );
    }
}
