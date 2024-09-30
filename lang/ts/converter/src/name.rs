use genotype_lang_ts_tree::{definition::TSDefinition, import_name::TSImportName, name::TSName};
use genotype_parser::tree::name::GTName;

use crate::convert::TSConvert;

impl TSConvert<TSName> for GTName {
    fn convert<HoistFn>(&self, _hoist: &HoistFn) -> TSName
    where
        HoistFn: Fn(TSDefinition),
    {
        TSName(self.0.clone())
    }
}

impl TSConvert<TSImportName> for GTName {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSImportName
    where
        HoistFn: Fn(TSDefinition),
    {
        TSImportName::Name(self.convert(hoist))
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::import_name::TSImportName;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::name::GTName;

    #[test]
    fn test_convert_name() {
        assert_eq!(
            TSName("Name".to_string()),
            GTName("Name".to_string()).convert(&|_| {}),
        );
    }

    #[test]
    fn test_convert_import_name() {
        assert_eq!(
            TSImportName::Name(TSName("Name".to_string())),
            GTName("Name".to_string()).convert(&|_| {}),
        );
    }
}
