use crate::{PYContext, PYContextResolve, PYDependency};

use super::PYNewtype;

impl PYContextResolve for PYNewtype {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYContext,
    {
        context.import(PYDependency::Typing, "NewType".into());
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
        let alias = PYNewtype {
            doc: None,
            name: "Foo".into(),
            primitive: PYPrimitive::String,
        };
        alias.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependency::Typing, "NewType".into())]
        );
    }
}
