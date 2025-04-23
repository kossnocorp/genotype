use crate::prelude::internal::*;

impl PYContextResolve for PYAlias {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYConvertContextConstraint,
    {
        if context.is_version(PYVersion::Legacy) {
            context.add_import(PYDependencyIdent::Typing, "TypeAlias".into());
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
        let alias = PYAlias {
            doc: None,
            name: "Foo".into(),
            descriptor: PYPrimitive::String.into(),
            references: vec![],
        };
        alias.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYConvertContextMock::new(PYVersion::Legacy);
        let alias = PYAlias {
            doc: None,
            name: "Foo".into(),
            descriptor: PYPrimitive::String.into(),
            references: vec![],
        };
        alias.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependencyIdent::Typing, "TypeAlias".into())]
        );
    }
}
