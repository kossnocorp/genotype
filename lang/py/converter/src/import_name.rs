use crate::prelude::internal::*;

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
    use super::*;
    use pretty_assertions::assert_eq;

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
