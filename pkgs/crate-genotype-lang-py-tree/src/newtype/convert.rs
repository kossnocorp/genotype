use crate::prelude::internal::*;

impl PyConvert<PyNewtype> for GtBranded {
    fn convert(&self, context: &mut PyConvertContext) -> PyNewtype {
        let doc = context.consume_doc();
        let name = self.name.convert(context);
        let primitive = self.primitive.convert(context);

        PyNewtype {
            doc,
            name,
            primitive,
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
            convert_node(
                Gt::branded("UserId", Gt::primitive_string())
            ),
            @r#"
        PyNewtype(
          doc: None,
          name: PyIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PyConvertContext::default();
        assert_ron_snapshot!(
            convert_node_with(
                Gt::branded("UserId", Gt::primitive_string()),
                &mut context
            ),
            @r#"
        PyNewtype(
          doc: None,
          name: PyIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Typing,
            reference: Named([
              Name(PyIdentifier("NewType")),
            ]),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = PyConvertContext::default();
        context.provide_doc(Some(PyDoc("Hello, world!".into())));
        assert_ron_snapshot!(
            convert_node_with(
                Gt::branded("UserId", Gt::primitive_string()),
                &mut context
            ),
            @r#"
        PyNewtype(
          doc: Some(PyDoc("Hello, world!")),
          name: PyIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
    }
}
