use crate::prelude::internal::*;

impl PYConvert<PYReference> for GTReference {
    fn convert(&self, context: &mut PYConvertContext) -> PYReference {
        let identifier = self.identifier.convert(context);
        let forward = context.is_forward_identifier(&identifier, &self.identifier);
        PYReference::new(identifier, forward)
    }
}

impl PYConvert<PYReference> for GTInlineImport {
    fn convert(&self, context: &mut PYConvertContext) -> PYReference {
        let name = self.name.convert(context);
        let path = self.path.convert(context);
        context.add_import(PYDependencyIdent::Path(path), name.clone());
        PYReference::new(name, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert_reference() {
        assert_ron_snapshot!(
            convert_to_py_with(
                GtFactory::reference("Name"),
                |context| {
                    context.push_defined(&"Name".into());
                }
            ),
            @r#"
        PYReference(
          identifier: PYIdentifier("Name"),
          forward: false,
        )
        "#,
        );
    }

    #[test]
    fn test_convert_reference_forward() {
        assert_ron_snapshot!(
            convert_to_py(GtFactory::reference("Name")),
            @r#"
        PYReference(
          identifier: PYIdentifier("Name"),
          forward: true,
        )
        "#,
        );
    }

    #[test]
    fn test_convert_inline_import() {
        let mut context = PYConvertContext::default();

        assert_ron_snapshot!(
            convert_to_py_with_context(GtFactory::inline_import("./path/to/module", "Name"), &mut context),
            @r#"
        PYReference(
          identifier: PYIdentifier("Name"),
          forward: false,
        )
        "#,
        );

        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Path(PYPath(".path.to.module")), PYIdentifier("Name")),
        ]
        "#
        );
    }
}
