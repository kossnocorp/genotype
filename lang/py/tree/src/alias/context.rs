use crate::{PYContext, PYContextResolve, PYVersion};

use super::PYAlias;

impl PYContextResolve for PYAlias {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYContext,
    {
        if context.is_version(PYVersion::Legacy) {
            context.import("typing".into(), "TypeAlias".into());
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
        let alias = PYAlias {
            name: "Foo".into(),
            descriptor: PYPrimitive::String.into(),
        };
        alias.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYContextMock::new(PYVersion::Legacy);
        let alias = PYAlias {
            name: "Foo".into(),
            descriptor: PYPrimitive::String.into(),
        };
        alias.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![("typing".into(), "TypeAlias".into())]
        );
    }
}
