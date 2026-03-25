use crate::prelude::internal::*;

impl PyContextResolve for PyAny {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PyConvertContextConstraint,
    {
        context.add_import(PyDependencyIdent::Typing, "Any".into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_resolve() {
        let mut context = PyConvertContextMock::default();
        let alias = PyAny;
        alias.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PyDependencyIdent::Typing, "Any".into())]
        );
    }
}
