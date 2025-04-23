use crate::prelude::internal::*;

impl PYContextResolve for PYProperty {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYConvertContextConstraint,
    {
        if !self.required {
            context.add_import(PYDependencyIdent::Typing, "Optional".into());
        }
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
        let alias = PYProperty {
            doc: None,
            name: "foo".into(),
            descriptor: PYPrimitive::String.into(),
            required: true,
        };
        alias.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_optional() {
        let mut context = PYConvertContextMock::default();
        let alias = PYProperty {
            doc: None,
            name: "foo".into(),
            descriptor: PYPrimitive::String.into(),
            required: false,
        };
        alias.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependencyIdent::Typing, "Optional".into())]
        );
    }
}
