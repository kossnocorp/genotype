use crate::prelude::internal::*;

impl PYContextResolve for PYList {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYConvertContextConstraint,
    {
        if context.is_version(PYVersion::Legacy) {
            context.add_import(PYDependencyIdent::Typing, "List".into());
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
        let list = PYList {
            descriptor: PYPrimitive::String.into(),
        };
        list.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYConvertContextMock::new(PYVersion::Legacy);
        let list = PYList {
            descriptor: PYPrimitive::String.into(),
        };
        list.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependencyIdent::Typing, "List".into())]
        );
    }
}
