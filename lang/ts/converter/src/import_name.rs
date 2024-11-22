use genotype_lang_ts_tree::import_name::TSImportName;
use genotype_parser::tree::import_name::GTImportName;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSImportName> for GTImportName {
    fn convert(&self, context: &mut TSConvertContext) -> TSImportName {
        match self {
            Self::Name(_, name) => TSImportName::Name(name.convert(context)),

            Self::Alias(_, name, alias) => {
                TSImportName::Alias(name.convert(context), alias.convert(context))
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
            GTImportName::Name(
                (0, 0).into(),
                GTIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&mut Default::default()),
            TSImportName::Name("Name".into()),
        );
    }

    #[test]
    fn test_convert_alias() {
        assert_eq!(
            GTImportName::Alias(
                (0, 0).into(),
                GTIdentifier::new((0, 0).into(), "Name".into()),
                GTIdentifier::new((0, 0).into(), "Alias".into())
            )
            .convert(&mut Default::default()),
            TSImportName::Alias("Name".into(), "Alias".into()),
        );
    }
}
