use crate::prelude::internal::*;

impl PyContextResolve for PyClass {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PyConvertContextConstraint,
    {
        context.add_import(PyDependencyIdent::Runtime, "Model".into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_resolve() {
        let mut context = PyConvertContextMock::default();
        let alias = PyClass {
            doc: None,
            name: "Foo".into(),
            extensions: vec![],
            properties: vec![],
            references: vec![],
        };
        alias.resolve(&mut context);
        assert_ron_snapshot!(
            context.as_imports(),
            @r#"
        [
          (Runtime, PyIdentifier("Model")),
        ]
        "#
        );
    }
}
