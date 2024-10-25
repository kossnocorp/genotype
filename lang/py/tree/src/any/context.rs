use crate::{PYContext, PYContextResolve, PYDependency};

use super::PYAny;

impl PYContextResolve for PYAny {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYContext,
    {
        context.import(PYDependency::Typing, "Any".into());
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
        let alias = PYAny;
        alias.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependency::Typing, "Any".into())]
        );
    }
}
