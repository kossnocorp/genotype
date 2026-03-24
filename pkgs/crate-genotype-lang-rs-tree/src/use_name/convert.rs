use crate::prelude::internal::*;

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
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_name() {
        assert_ron_snapshot!(
            GTImportName::Name(
                (0, 0).into(),
                GTIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @r#"Name(RSIdentifier("Name"))"#
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
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @r#"Alias(RSIdentifier("Name"), RSIdentifier("Alias"))"#
        );
    }
}
