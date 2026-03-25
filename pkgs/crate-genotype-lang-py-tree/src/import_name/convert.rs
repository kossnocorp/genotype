use crate::prelude::internal::*;

impl PyConvert<PyImportName> for GtImportName {
    fn convert(&self, context: &mut PyConvertContext) -> PyImportName {
        match self {
            Self::Name(_, name) => PyImportName::Name(name.convert(context)),

            Self::Alias(_, name, alias) => {
                PyImportName::Alias(name.convert(context), alias.convert(context))
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
            GtImportName::Name(
                (0, 0).into(),
                GtIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&mut PyConvertContext::default()),
            @r#"Name(PyIdentifier("Name"))"#,
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
            .convert(&mut PyConvertContext::default()),
            @r#"Alias(PyIdentifier("Name"), PyIdentifier("Alias"))"#,
        );
    }
}
