use crate::prelude::internal::*;

impl PyConvert<PyReference> for GtReference {
    fn convert(&self, context: &mut PyConvertContext) -> PyReference {
        let identifier = self.identifier.convert(context);
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.descriptor.convert(context))
            .collect::<Vec<_>>();
        let forward = !context.is_generic_parameter(&identifier)
            && context.is_forward_identifier(&identifier, &self.identifier);
        PyReference::new_with_arguments(identifier, arguments, forward)
    }
}

impl PyConvert<PyReference> for GtInlineImport {
    fn convert(&self, context: &mut PyConvertContext) -> PyReference {
        let name = self.name.convert(context);
        let path = self.path.convert(context);
        context.push_import(PyImport::new(
            PyDependencyIdent::Local(path),
            PyImportReference::Named(vec![name.clone().into()]),
        ));
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.descriptor.convert(context))
            .collect::<Vec<_>>();
        PyReference::new_with_arguments(name, arguments, false)
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
          arguments: [],
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
          arguments: [],
          forward: true,
        )
        "#,
        );
    }

    #[test]
    fn test_convert_reference_with_arguments() {
        let mut context = PyConvertContext::default();
        context.push_defined(&"Response".into());
        let reference = GtReference {
            arguments: vec![Gt::primitive_string().into()],
            ..Gt::reference_anon("Response")
        };

        assert_ron_snapshot!(
            convert_node_with(reference, &mut context),
            @r#"
        PyReference(
          identifier: PyIdentifier("Response"),
          arguments: [
            Primitive(String),
          ],
          forward: false,
        )
        "#,
        );
    }

    #[test]
    fn test_convert_generic_parameter() {
        let mut context = PyConvertContext::default();
        context.enter_generics_scope(vec!["Payload".into()]);

        assert_ron_snapshot!(
            convert_node_with(Gt::reference_anon("Payload"), &mut context),
            @r#"
        PyReference(
          identifier: PyIdentifier("Payload"),
          arguments: [],
          forward: false,
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
          arguments: [],
          forward: false,
        )
        "#,
        );

        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Local(PyPath(".path.to.module")),
            reference: Named([
              Name(PyIdentifier("Name")),
            ]),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_inline_import_with_arguments() {
        let mut context = PyConvertContext::default();
        let import = GtInlineImport {
            arguments: vec![Gt::primitive_string().into(), Gt::primitive_number().into()],
            ..Gt::inline_import_anon("./path/to/module", "Pair")
        };

        assert_ron_snapshot!(
            convert_node_with(import, &mut context),
            @r#"
        PyReference(
          identifier: PyIdentifier("Pair"),
          arguments: [
            Primitive(String),
            Primitive(Float),
          ],
          forward: false,
        )
        "#,
        );
    }
}
