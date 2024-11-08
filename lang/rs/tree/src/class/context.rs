use crate::{RSContext, RSContextResolve, RSDependency};

use super::RSClass;

impl RSContextResolve for RSClass {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: RSContext,
    {
        context.import(RSDependency::Runtime, "Model".into());
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
        let alias = RSClass {
            doc: None,
            name: "Foo".into(),
            extensions: vec![],
            properties: vec![],
            references: vec![],
        };
        alias.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(RSDependency::Runtime, "Model".into())]
        );
    }
}
