use genotype_lang_py_tree::import_name::PYImportName;
use genotype_parser::tree::import_name::GTImportName;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYImportName> for GTImportName {
    fn convert(&self, context: &mut PYConvertContext) -> PYImportName {
        match self {
            Self::Name(_, name) => PYImportName::Name(name.convert(context)),

            Self::Alias(_, name, alias) => {
                PYImportName::Alias(name.convert(context), alias.convert(context))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::import_name::PYImportName;
    use pretty_assertions::assert_eq;

    use crate::context::PYConvertContext;

    use super::*;
    use genotype_parser::{tree::import_name::GTImportName, GTIdentifier};

    #[test]
    fn test_convert_name() {
        assert_eq!(
            GTImportName::Name(
                (0, 0).into(),
                GTIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&mut PYConvertContext::default()),
            PYImportName::Name("Name".into()),
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
            .convert(&mut PYConvertContext::default()),
            PYImportName::Alias("Name".into(), "Alias".into()),
        );
    }
}
