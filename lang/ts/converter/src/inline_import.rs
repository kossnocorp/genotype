use genotype_lang_ts_tree::{definition::TSDefinition, inline_import::TSInlineImport};
use genotype_parser::tree::inline_import::GTInlineImport;

use crate::convert::TSConvert;

impl TSConvert<TSInlineImport> for GTInlineImport {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSInlineImport
    where
        HoistFn: Fn(TSDefinition),
    {
        TSInlineImport {
            path: self.path.clone(),
            name: self.name.convert(hoist),
        }
    }
}

#[cfg(test)]
mod tests {

    use genotype_lang_ts_tree::name::TSName;
    use genotype_parser::tree::name::GTName;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTInlineImport {
                path: "./path/to/module".to_string(),
                name: GTName("Name".to_string()),
            }
            .convert(&|_| {}),
            TSInlineImport {
                path: "./path/to/module".to_string(),
                name: TSName("Name".to_string()),
            }
        );
    }
}
