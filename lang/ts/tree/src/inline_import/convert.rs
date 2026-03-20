use crate::prelude::internal::*;

impl TSConvert<TSInlineImport> for GTInlineImport {
    fn convert(&self, context: &mut TSConvertContext) -> TSInlineImport {
        TSInlineImport {
            path: self.path.convert(context),
            name: self.name.convert(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;
    use crate::test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_to_ts(GtFactory::inline_import("./path/to/module", "Name")),
            @r#"
        TSInlineImport(
          path: TSPath("./path/to/module"),
          name: TSIdentifier("Name"),
        )
        "#
        );
    }
}
