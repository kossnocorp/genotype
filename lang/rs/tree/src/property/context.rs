use crate::*;

use super::RSProperty;

impl RSContextResolve for RSProperty {
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
        let alias = RSProperty {
            doc: None,
            name: "foo".into(),
            descriptor: RSPrimitive::String.into(),
        };
        alias.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }
}
