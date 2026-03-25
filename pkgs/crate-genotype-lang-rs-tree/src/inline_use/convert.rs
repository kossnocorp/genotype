use crate::prelude::internal::*;

impl RsConvert<RsInlineUse> for GtInlineImport {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsInlineUse> {
        let path = self.path.convert(context)?;
        let name = self.name.convert(context)?;
        Ok(RsInlineUse { path, name })
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
            convert_node(Gt::inline_import("./path/to/module", "Name")),
            @r#"
        RsInlineUse(
          path: RsPath(GtModuleId("path/to/module"), "super::path::to::module"),
          name: RsIdentifier("Name"),
        )
        "#
        );
    }
}
