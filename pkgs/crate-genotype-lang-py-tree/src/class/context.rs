use crate::prelude::internal::*;

impl PYContextResolve for PYClass {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYConvertContextConstraint,
    {
        context.add_import(PYDependencyIdent::Runtime, "Model".into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_resolve() {
        let mut context = PYConvertContextMock::default();
        let alias = PYClass {
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
          (Runtime, PYIdentifier("Model")),
        ]
        "#
        );
    }
}
