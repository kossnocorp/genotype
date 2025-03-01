use crate::*;

use super::PYLiteral;

impl PYContextResolve for PYLiteral {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYContext,
    {
        context.import(PYDependency::Typing, "Literal".into());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use mock::PYContextMock;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_resolve() {
        let mut context = PYContextMock::default();
        let literal = PYLiteral::Boolean(true);
        literal.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependency::Typing, "Literal".into())]
        );
    }
}
