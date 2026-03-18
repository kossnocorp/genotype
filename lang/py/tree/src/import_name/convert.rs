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
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_name() {
        assert_ron_snapshot!(
            GTImportName::Name(
                (0, 0).into(),
                GTIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&mut PYConvertContext::default()),
            @r#"Name(PYIdentifier("Name"))"#,
        );
    }

    #[test]
    fn test_convert_alias() {
        assert_ron_snapshot!(
            GTImportName::Alias(
                (0, 0).into(),
                GTIdentifier::new((0, 0).into(), "Name".into()),
                GTIdentifier::new((0, 0).into(), "Alias".into())
            )
            .convert(&mut PYConvertContext::default()),
            @r#"Alias(PYIdentifier("Name"), PYIdentifier("Alias"))"#,
        );
    }
}
