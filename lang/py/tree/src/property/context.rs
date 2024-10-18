use crate::*;

use super::PYProperty;

impl PYContextResolve for PYProperty {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYContext,
    {
        if !self.required {
            context.import(PYDependency::Typing, "Optional".into());
        }
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
        let alias = PYProperty {
            name: "foo".into(),
            descriptor: PYPrimitive::String.into(),
            required: true,
        };
        alias.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_optional() {
        let mut context = PYContextMock::default();
        let alias = PYProperty {
            name: "foo".into(),
            descriptor: PYPrimitive::String.into(),
            required: false,
        };
        alias.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependency::Typing, "Optional".into())]
        );
    }
}
