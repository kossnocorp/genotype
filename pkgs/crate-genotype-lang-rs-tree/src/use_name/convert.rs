use crate::prelude::internal::*;

impl RsConvert<RsUseName> for GtImportName {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsUseName> {
        Ok(match self {
            Self::Name(_, name) => RsUseName::Name(name.convert(context)?),

            Self::Alias(_, name, alias) => {
                let name = name.convert(context)?;
                let alias = alias.convert(context)?;
                RsUseName::Alias(name, alias)
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
            GtImportName::Name(
                (0, 0).into(),
                GtIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @r#"Name(RsIdentifier("Name"))"#
        );
    }

    #[test]
    fn test_convert_alias() {
        assert_ron_snapshot!(
            GtImportName::Alias(
                (0, 0).into(),
                GtIdentifier::new((0, 0).into(), "Name".into()),
                GtIdentifier::new((0, 0).into(), "Alias".into())
            )
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @r#"Alias(RsIdentifier("Name"), RsIdentifier("Alias"))"#
        );
    }
}
