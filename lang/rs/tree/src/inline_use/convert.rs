use crate::prelude::internal::*;

impl RSConvert<RSInlineUse> for GTInlineImport {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSInlineUse> {
        let path = self.path.convert(context)?;
        let name = self.name.convert(context)?;
        Ok(RSInlineUse { path, name })
    }
}

#[cfg(test)]
mod tesrs {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::new(
                    (0, 0).into(),
                    GTPathModuleId::Resolved("module/path".into()),
                    "./path/to/module".into()
                ),
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        RSInlineUse(
          path: RSPath(GTModuleId("module/path"), "super::path::to::module"),
          name: RSIdentifier("Name"),
        )
        "#
        );
    }
}
