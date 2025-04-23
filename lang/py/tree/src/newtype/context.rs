use crate::prelude::internal::*;

impl PYContextResolve for PYNewtype {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYConvertContextConstraint,
    {
        context.add_import(PYDependencyIdent::Typing, "NewType".into());
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
        let alias = PYNewtype {
            doc: None,
            name: "Foo".into(),
            primitive: PYPrimitive::String,
        };
        alias.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependencyIdent::Typing, "NewType".into())]
        );
    }
}
