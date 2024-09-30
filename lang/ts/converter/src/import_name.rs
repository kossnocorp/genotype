use genotype_lang_ts_tree::{definition::TSDefinition, import_name::TSImportName};
use genotype_parser::tree::import_name::GTImportName;

use crate::convert::TSConvert;

impl TSConvert<TSImportName> for GTImportName {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSImportName
    where
        HoistFn: Fn(TSDefinition),
    {
        match self {
            Self::Name(name) => TSImportName::Name(name.convert(hoist)),

            Self::Alias(name, alias) => {
                TSImportName::Alias(name.convert(hoist), alias.convert(hoist))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::{import_name::TSImportName, name::TSName};
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::{import_name::GTImportName, name::GTName};

    #[test]
    fn test_convert_name() {
        assert_eq!(
            GTImportName::Name(GTName("Name".to_string())).convert(&|_| {}),
            TSImportName::Name(TSName("Name".to_string())),
        );
    }

    #[test]
    fn test_convert_alias() {
        assert_eq!(
            GTImportName::Alias(GTName("Name".to_string()), GTName("Alias".to_string()))
                .convert(&|_| {}),
            TSImportName::Alias(TSName("Name".to_string()), TSName("Alias".to_string())),
        );
    }
}
