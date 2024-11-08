use crate::*;

use super::RSProperty;

impl RSContextResolve for RSProperty {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: RSContext,
    {
        if !self.required {
            context.import(RSDependency::Typing, "Optional".into());
        }
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
            required: true,
        };
        alias.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_optional() {
        let mut context = RSContextMock::default();
        let alias = RSProperty {
            doc: None,
            name: "foo".into(),
            descriptor: RSPrimitive::String.into(),
            required: false,
        };
        alias.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(RSDependency::Typing, "Optional".into())]
        );
    }
}
