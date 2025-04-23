use crate::prelude::internal::*;

impl PYContextResolve for PYDict {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYConvertContextConstraint,
    {
        if context.is_version(PYVersion::Legacy) {
            context.add_import(PYDependencyIdent::Typing, "Dict".into());
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
        let dict = PYDict {
            key: PYDictKey::String,
            descriptor: PYPrimitive::String.into(),
        };
        dict.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYConvertContextMock::new(PYVersion::Legacy);
        let dict = PYDict {
            key: PYDictKey::String,
            descriptor: PYPrimitive::String.into(),
        };
        dict.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependencyIdent::Typing, "Dict".into())],
        );
    }
}
