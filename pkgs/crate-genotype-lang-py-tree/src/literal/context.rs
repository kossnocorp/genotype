use crate::prelude::internal::*;

impl PyContextResolve for PyLiteral {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PyConvertContextConstraint,
    {
        context.add_import(PyDependencyIdent::Typing, "Literal".into());
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
        let literal = PyLiteral::Boolean(true);
        literal.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PyDependencyIdent::Typing, "Literal".into())]
        );
    }
}
