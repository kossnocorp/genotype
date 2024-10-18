use crate::{PYContext, PYContextResolve, PYVersion};

use super::PYList;

impl PYContextResolve for PYList {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYContext,
    {
        if context.is_version(PYVersion::Legacy) {
            context.import("typing".into(), "List".into());
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
        let list = PYList {
            descriptor: PYPrimitive::String.into(),
        };
        list.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYContextMock::new(PYVersion::Legacy);
        let list = PYList {
            descriptor: PYPrimitive::String.into(),
        };
        list.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![("typing".into(), "List".into())]);
    }
}
