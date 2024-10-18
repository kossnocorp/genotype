use crate::*;

use super::PYUnion;

impl PYContextResolve for PYUnion {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYContext,
    {
        if context.is_version(PYVersion::Legacy) {
            context.import("typing".into(), "Union".into());
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
        let union = PYUnion {
            descriptors: vec![PYPrimitive::String.into()],
        };
        union.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYContextMock::new(PYVersion::Legacy);
        let union = PYUnion {
            descriptors: vec![PYPrimitive::String.into()],
        };
        union.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![("typing".into(), "Union".into())]
        );
    }
}
