use crate::prelude::internal::*;

impl PyConvert<PyReference> for GtReference {
    fn convert(&self, context: &mut PyConvertContext) -> PyReference {
        let identifier = self.identifier.convert(context);
        let forward = context.is_forward_identifier(&identifier, &self.identifier);
        PyReference::new(identifier, forward)
    }
}

impl PyConvert<PyReference> for GtInlineImport {
    fn convert(&self, context: &mut PyConvertContext) -> PyReference {
        let name = self.name.convert(context);
        let path = self.path.convert(context);
        context.push_import(PyImport::new(
            PyDependencyIdent::Path(path),
            PyImportReference::Named(vec![name.clone().into()]),
        ));
        PyReference::new(name, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use genotype_test::*;

    #[test]
    fn test_convert_reference() {
        let mut context = PyConvertContext::default();
        context.push_defined(&"Name".into());
        assert_ron_snapshot!(
            convert_node_with(Gt::reference_anon("Name"), &mut context),
            @r#"
        PyReference(
          identifier: PyIdentifier("Name"),
          forward: false,
        )
        "#,
        );
    }

    #[test]
    fn test_convert_reference_forward() {
        assert_ron_snapshot!(
            convert_node(Gt::reference_anon("Name")),
            @r#"
        PyReference(
          identifier: PyIdentifier("Name"),
          forward: true,
        )
        "#,
        );
    }

    #[test]
    fn test_convert_inline_import() {
        let mut context = PyConvertContext::default();

        assert_ron_snapshot!(
            convert_node_with(Gt::inline_import_anon("./path/to/module", "Name"), &mut context),
            @r#"
        PyReference(
          identifier: PyIdentifier("Name"),
          forward: false,
        )
        "#,
        );

        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Path(PyPath(".path.to.module")),
            reference: Named([
              Name(PyIdentifier("Name")),
            ]),
          ),
        ]
        "#
        );
    }
}
