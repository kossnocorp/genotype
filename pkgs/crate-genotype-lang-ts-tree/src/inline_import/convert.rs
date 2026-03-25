use crate::prelude::internal::*;

impl TsConvert<TsInlineImport> for GtInlineImport {
    fn convert(&self, context: &mut TsConvertContext) -> TsInlineImport {
        TsInlineImport {
            path: self.path.convert(context),
            name: self.name.convert(context),
        }
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
        TsInlineImport(
          path: TsPath("./path/to/module"),
          name: TsIdentifier("Name"),
        )
        "#
        );
    }
}
