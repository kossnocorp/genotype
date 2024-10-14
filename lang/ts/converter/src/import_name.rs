use genotype_lang_ts_tree::{definition::TSDefinition, import_name::TSImportName};
use genotype_parser::tree::import_name::GTImportName;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSImportName> for GTImportName {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSImportName
    where
        HoistFn: Fn(TSDefinition),
    {
        match self {
            Self::Name(name) => TSImportName::Name(name.convert(resolve, hoist)),

            Self::Alias(name, alias) => {
                TSImportName::Alias(name.convert(resolve, hoist), alias.convert(resolve, hoist))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::import_name::TSImportName;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::{tree::import_name::GTImportName, GTIdentifier};

    #[test]
    fn test_convert_name() {
        assert_eq!(
            GTImportName::Name(GTIdentifier::new((0, 0).into(), "Name".into()))
                .convert(&TSConvertResolve::new(), &|_| {}),
            TSImportName::Name("Name".into()),
        );
    }

    #[test]
    fn test_convert_alias() {
        assert_eq!(
            GTImportName::Alias(
                GTIdentifier::new((0, 0).into(), "Name".into()),
                GTIdentifier::new((0, 0).into(), "Alias".into())
            )
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSImportName::Alias("Name".into(), "Alias".into()),
        );
    }
}
