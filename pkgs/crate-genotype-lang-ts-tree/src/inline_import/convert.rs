use crate::prelude::internal::*;

impl TsConvert<TsInlineImport> for GtInlineImport {
    fn convert(&self, context: &mut TsConvertContext) -> TsInlineImport {
        let path = self.path.convert(context);
        let name = self.name.convert(context);
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.descriptor.convert(context))
            .collect();

        if context.is_zod_mode() {
            context.push_import(TsImport::new(
                TsDependencyIdent::Local(path.clone()),
                TsImportReference::Named(vec![TsImportName::Name(name.clone())]),
            ));
        }

        TsInlineImport {
            path,
            name,
            arguments,
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
            convert_node(Gt::inline_import_anon("./path/to/module", "Name")),
            @r#"
        TsInlineImport(
          path: TsPath("./path/to/module"),
          name: TsIdentifier("Name"),
          arguments: [],
        )
        "#
        );
    }

    #[test]
    fn test_convert_with_arguments() {
        assert_ron_snapshot!(
            convert_node(GtInlineImport {
                arguments: vec![GtGenericArgument {
                    span: (0, 0).into(),
                    descriptor: Gt::primitive_string().into(),
                }],
                ..Gt::inline_import_anon("./path/to/module", "Name")
            }),
            @r#"
        TsInlineImport(
          path: TsPath("./path/to/module"),
          name: TsIdentifier("Name"),
          arguments: [
            Primitive(String),
          ],
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
          arguments: [],
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
          arguments: [],
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
