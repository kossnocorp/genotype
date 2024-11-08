use crate::*;

use super::RSLiteral;

impl RSContextResolve for RSLiteral {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: RSContext,
    {
        context.import(RSDependency::Typing, "Literal".into());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use mock::RSContextMock;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_resolve() {
        let mut context = RSContextMock::default();
        let literal = RSLiteral::Boolean(true);
        literal.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(RSDependency::Typing, "Literal".into())]
        );
    }
}
