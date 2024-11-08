use genotype_lang_rs_tree::import_name::RSImportName;
use genotype_parser::tree::import_name::GTImportName;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSImportName> for GTImportName {
    fn convert(&self, context: &mut RSConvertContext) -> RSImportName {
        match self {
            Self::Name(_, name) => RSImportName::Name(name.convert(context)),

            Self::Alias(_, name, alias) => {
                RSImportName::Alias(name.convert(context), alias.convert(context))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::import_name::RSImportName;
    use pretty_assertions::assert_eq;

    use crate::context::RSConvertContext;

    use super::*;
    use genotype_parser::{tree::import_name::GTImportName, GTIdentifier};

    #[test]
    fn test_convert_name() {
        assert_eq!(
            GTImportName::Name(
                (0, 0).into(),
                GTIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&mut RSConvertContext::default()),
            RSImportName::Name("Name".into()),
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
            .convert(&mut RSConvertContext::default()),
            RSImportName::Alias("Name".into(), "Alias".into()),
        );
    }
}
