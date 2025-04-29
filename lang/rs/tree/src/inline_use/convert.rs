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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(
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
            RSInlineUse {
                path: RSPath("module/path".into(), "super::path::to::module".into()),
                name: "Name".into(),
            }
        );
    }
}
