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
        let mut resolve = RsConvertResolve::default();
        resolve.path_module_ids.insert(
            GtPathModuleId::new((0, 0).into(), "module".into()),
            "module/path".into(),
        );
        let mut context = Rst::convert_context_with_resolve(resolve);
        assert_ron_snapshot!(
            convert_node_with(
                Gt::inline_import("./path/to/module", "Name"),
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
