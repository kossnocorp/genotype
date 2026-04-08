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

    #[test]
    fn test_convert() {
        let mut context = Rst::convert_context_with(
            vec![(Gt::path_module_id((0, 0)), "module/path".into())],
            vec![],
        );
        assert_ron_snapshot!(
            convert_node_with(
                Gt::inline_import_anon("./path/to/module", "Name"),
                &mut context
            ),
            @r#"
        RsInlineUse(
          path: RsPath(GtModuleId("module/path"), "super::path::to::module"),
          name: RsIdentifier("Name"),
        )
        "#
        );
    }
}
