use crate::{RSContext, RSContextResolve, RSDependency};

use super::RSAny;

impl RSContextResolve for RSAny {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: RSContext,
    {
        context.import(RSDependency::Typing, "Any".into());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use mock::RSContextMock;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_resolve() {
        let mut context = RSContextMock::default();
        let alias = RSAny;
        alias.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(RSDependency::Typing, "Any".into())]
        );
    }
}
