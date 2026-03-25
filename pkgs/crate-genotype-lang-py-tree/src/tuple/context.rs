use crate::prelude::internal::*;

impl PyContextResolve for PyTuple {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PyConvertContextConstraint,
    {
        if context.is_version(PyVersion::Legacy) {
            context.add_import(PyDependencyIdent::Typing, "Tuple".into());
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
        let tuple = PyTuple {
            descriptors: vec![PyPrimitive::String.into()],
        };
        tuple.resolve(&mut context);
        assert_ron_snapshot!(context.as_imports(), @"[]");
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PyConvertContextMock::new(PyVersion::Legacy);
        let tuple = PyTuple {
            descriptors: vec![PyPrimitive::String.into()],
        };
        tuple.resolve(&mut context);
        assert_ron_snapshot!(
            context.as_imports(),
            @r#"
        [
          (Typing, PyIdentifier("Tuple")),
        ]
        "#
        );
    }
}
