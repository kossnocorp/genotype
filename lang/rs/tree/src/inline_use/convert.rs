use crate::prelude::internal::*;

impl RSConvert<RSInlineUse> for GTInlineImport {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSInlineUse> {
        let path = self.path.convert(context)?;
        let name = self.name.convert(context)?;
        Ok(RSInlineUse { path, name })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_to_rs(GtFactory::inline_import("./path/to/module", "Name")),
            @r#"
        RSInlineUse(
          path: RSPath(GTModuleId("path/to/module"), "super::path::to::module"),
          name: RSIdentifier("Name"),
        )
        "#
        );
    }
}
