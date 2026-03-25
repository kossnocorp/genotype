use crate::prelude::internal::*;

impl PyContextResolve for PyProperty {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PyConvertContextConstraint,
    {
        if !self.required {
            context.add_import(PyDependencyIdent::Typing, "Optional".into());
        }
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
        let alias = PyProperty {
            doc: None,
            name: "foo".into(),
            descriptor: PyPrimitive::String.into(),
            required: true,
        };
        alias.resolve(&mut context);
        assert_ron_snapshot!(context.as_imports(), @"[]");
    }

    #[test]
    fn test_resolve_optional() {
        let mut context = PyConvertContextMock::default();
        let alias = PyProperty {
            doc: None,
            name: "foo".into(),
            descriptor: PyPrimitive::String.into(),
            required: false,
        };
        alias.resolve(&mut context);
        assert_ron_snapshot!(
            context.as_imports(),
            @r#"
        [
          (Typing, PyIdentifier("Optional")),
        ]
        "#
        );
    }
}
