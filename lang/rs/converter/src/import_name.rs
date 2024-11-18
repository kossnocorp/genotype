use genotype_lang_rs_tree::use_name::RSUseName;
use genotype_parser::tree::import_name::GTImportName;
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSUseName> for GTImportName {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSUseName> {
        Ok(match self {
            Self::Name(_, name) => RSUseName::Name(name.convert(context)?),

            Self::Alias(_, name, alias) => {
                let name = name.convert(context)?;
                let alias = alias.convert(context)?;
                RSUseName::Alias(name, alias)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::use_name::RSUseName;
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
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSUseName::Name("Name".into()),
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
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSUseName::Alias("Name".into(), "Alias".into()),
        );
    }
}
