use crate::prelude::internal::*;

impl PyContextResolve for PyClass {
    fn resolve(self, context: &mut PyConvertContext) -> Self {
        context.push_import(PyImport::new(PyDependencyIdent::Runtime, "Model".into()));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_resolve() {
        let mut context = PyConvertContext::default();
        let alias = PyClass {
            doc: None,
            name: "Foo".into(),
            extensions: vec![],
            properties: vec![],
            references: vec![],
        };
        alias.resolve(&mut context);
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Runtime,
            reference: Named([
              Name(PyIdentifier("Model")),
            ]),
          ),
        ]
        "#
        );
    }
}
