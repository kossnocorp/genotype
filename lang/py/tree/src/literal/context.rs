use crate::prelude::internal::*;

impl PYContextResolve for PYLiteral {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYConvertContextConstraint,
    {
        context.add_import(PYDependencyIdent::Typing, "Literal".into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_resolve() {
        let mut context = PYConvertContextMock::default();
        let literal = PYLiteral::Boolean(true);
        literal.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependencyIdent::Typing, "Literal".into())]
        );
    }
}
