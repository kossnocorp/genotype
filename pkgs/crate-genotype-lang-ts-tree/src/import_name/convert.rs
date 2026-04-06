use crate::prelude::internal::*;

impl TsConvert<TsImportName> for GtImportName {
    fn convert(&self, context: &mut TsConvertContext) -> TsImportName {
        match self {
            Self::Name(_, name) => TsImportName::Name(name.convert(context)),

            Self::Alias(_, name, alias) => {
                TsImportName::Alias(name.convert(context), alias.convert(context))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_name() {
        assert_ron_snapshot!(
            convert_node(GtImportName::Name(
                (0, 0).into(),
                GtIdentifier::new((0, 0).into(), "Name".into())
            )),
            @r#"Name(TsIdentifier("Name"))"#,
        );
    }

    #[test]
    fn test_convert_alias() {
        assert_ron_snapshot!(
            convert_node(GtImportName::Alias(
                (0, 0).into(),
                GtIdentifier::new((0, 0).into(), "Name".into()),
                GtIdentifier::new((0, 0).into(), "Alias".into())
            )),
            @r#"Alias(TsIdentifier("Name"), TsIdentifier("Alias"))"#,
        );
    }
}
