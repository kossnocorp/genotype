use crate::prelude::internal::*;

impl PyConvert<PyClass> for GtObject {
    fn convert(&self, context: &mut PyConvertContext) -> PyClass {
        context.create_references_scope();

        let name = match &self.name {
            GtObjectName::Named(identifier) => identifier.convert(context),
            GtObjectName::Alias(identifier, _) => identifier.convert(context),
        };

        let doc = context.consume_doc();
        let extensions = self.extensions.iter().map(|e| e.convert(context)).collect();
        let properties = self.properties.iter().map(|p| p.convert(context)).collect();

        let references = context.pop_references_scope();

        PyClass {
            doc,
            name,
            extensions,
            properties,
            references,
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GtObject {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtObjectName::Named(GtIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![
                    GtProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GtKey::new((0, 0).into(), "name".into()),
                        descriptor: Gt::primitive_string().into(),
                        required: true,
                    },
                    GtProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GtKey::new((0, 0).into(), "age".into()),
                        descriptor: Gt::primitive_i32().into(),
                        required: false,
                    }
                ]
            }
            .convert(&mut PyConvertContext::default()),
            @r#"
        PyClass(
          doc: None,
          name: PyIdentifier("Person"),
          extensions: [],
          properties: [
            PyProperty(
              doc: None,
              name: PyKey("name"),
              descriptor: Primitive(String),
              required: true,
            ),
            PyProperty(
              doc: None,
              name: PyKey("age"),
              descriptor: Primitive(Int),
              required: false,
            ),
          ],
          references: [],
        )
        "#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PyConvertContext::default();
        assert_ron_snapshot!(
            GtObject {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtObjectName::Named(GtIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![]
            }
            .convert(&mut context),
            @r#"
        PyClass(
          doc: None,
          name: PyIdentifier("Person"),
          extensions: [],
          properties: [],
          references: [],
        )
        "#
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Runtime, PyIdentifier("Model")),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = PyConvertContext::default();
        context.provide_doc(Some(PyDoc("Hello, world!".into())));
        assert_ron_snapshot!(
            GtObject {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtObjectName::Named(GtIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![],
            }
            .convert(&mut context),
            @r#"
        PyClass(
          doc: Some(PyDoc("Hello, world!")),
          name: PyIdentifier("Person"),
          extensions: [],
          properties: [],
          references: [],
        )
        "#
        );
    }
}
