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
    use genotype_parser::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSInlineImport {
                path: "./path/to/module.ts".into(),
                name: "Name".into(),
            }
        );
    }
}
