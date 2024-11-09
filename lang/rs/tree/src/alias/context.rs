use crate::{RSContext, RSContextResolve};

use super::RSAlias;

impl RSContextResolve for RSAlias {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: RSContext,
    {
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
        let alias = RSAlias {
            doc: None,
            name: "Foo".into(),
            descriptor: RSPrimitive::String.into(),
        };
        alias.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }
}
