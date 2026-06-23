use crate::prelude::internal::*;

impl RsConvert<RsInlineUse> for GtInlineImport {
    fn convert(&self, context: &mut RsConvertContext) -> RsConvertResult<RsInlineUse> {
        let path = self.path.convert(context)?;
        let name = self.name.convert(context)?;
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.descriptor.convert(context))
            .collect::<RsConvertResult<Vec<_>>>()?;
        Ok(RsInlineUse {
            path,
            name,
            arguments,
        })
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
          arguments: [],
        )
        "#
        );
    }

    #[test]
    fn test_convert_with_arguments() {
        let mut context = Rst::convert_context_with(
            vec![(Gt::path_module_id((0, 0)), "module/path".into())],
            vec![],
        );
        let import = GtInlineImport {
            arguments: vec![Gt::primitive_string().into(), Gt::primitive_number().into()],
            ..Gt::inline_import_anon("./path/to/module", "Pair")
        };

        assert_ron_snapshot!(
            convert_node_with(import, &mut context),
            @r#"
        RsInlineUse(
          path: RsPath(GtModuleId("module/path"), "super::path::to::module"),
          name: RsIdentifier("Pair"),
          arguments: [
            Primitive(String),
            Primitive(Float64),
          ],
        )
        "#,
        );
    }
}
