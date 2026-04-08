use crate::prelude::internal::*;

impl TsConvert<TsInlineImport> for GtInlineImport {
    fn convert(&self, context: &mut TsConvertContext) -> TsInlineImport {
        let path = self.path.convert(context);
        let name = self.name.convert(context);

        if context.is_zod_mode() {
            context.push_import(TsImport::new(
                TsDependencyIdent::Local(path.clone()),
                TsImportReference::Named(vec![TsImportName::Name(name.clone())]),
            ));
        }

        TsInlineImport { path, name }
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
            convert_node(Gt::inline_import_anon("./path/to/module", "Name")),
            @r#"
        TsInlineImport(
          path: TsPath("./path/to/module"),
          name: TsIdentifier("Name"),
        )
        "#
        );
    }

    #[test]
    fn test_convert_does_not_add_import_dependency() {
        let mut context = TsConvertContext::default();

        assert_ron_snapshot!(
            convert_node_with(Gt::inline_import_anon("./path/to/module", "Name"), &mut context),
            @r#"
        TsInlineImport(
          path: TsPath("./path/to/module"),
          name: TsIdentifier("Name"),
        )
        "#
        );

        assert_ron_snapshot!(
            context.drain_imports(),
            @"[]"
        );
    }

    #[test]
    fn test_convert_adds_import_dependency_in_zod_mode() {
        let mut context = Tst::convert_context_zod();

        assert_ron_snapshot!(
            convert_node_with(Gt::inline_import_anon("./path/to/module", "Name"), &mut context),
            @r#"
        TsInlineImport(
          path: TsPath("./path/to/module"),
          name: TsIdentifier("Name"),
        )
        "#
        );

        assert_ron_snapshot!(
            context.drain_imports(),
            @r#"
        [
          TsImport(
            dependency: Local(TsPath("./path/to/module")),
            reference: Named([
              Name(TsIdentifier("Name")),
            ]),
          ),
        ]
        "#
        );
    }
}
