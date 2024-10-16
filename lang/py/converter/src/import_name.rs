use genotype_lang_py_tree::{definition::PYDefinition, import_name::PYImportName};
use genotype_parser::tree::import_name::GTImportName;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYImportName> for GTImportName {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYImportName
    where
        HoistFn: Fn(PYDefinition),
    {
        match self {
            Self::Name(_, name) => PYImportName::Name(name.convert(resolve, hoist)),

            Self::Alias(_, name, alias) => {
                PYImportName::Alias(name.convert(resolve, hoist), alias.convert(resolve, hoist))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::import_name::PYImportName;
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
            .convert(&PYConvertResolve::new(), &|_| {}),
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
            .convert(&PYConvertResolve::new(), &|_| {}),
            PYImportName::Alias("Name".into(), "Alias".into()),
        );
    }
}
