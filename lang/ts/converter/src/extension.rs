use genotype_lang_ts_tree::{definition::TSDefinition, TSExtension};
use genotype_parser::tree::GTExtension;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSExtension> for GTExtension {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSExtension
    where
        HoistFn: Fn(TSDefinition),
    {
        TSExtension {
            reference: self.reference.convert(resolve, hoist),
        }
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
            TSExtension {
                reference: "Name".into()
            },
            GTExtension {
                reference: "Name".into()
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
        );
    }
}