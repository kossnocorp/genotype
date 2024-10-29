use crate::{PYContext, PYContextResolve, PYDependency};

use super::PYClass;

impl PYContextResolve for PYClass {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYContext,
    {
        context.import(PYDependency::Runtime, "Model".into());
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
        let alias = PYClass {
            doc: None,
            name: "Foo".into(),
            extensions: vec![],
            properties: vec![],
            references: vec![],
        };
        alias.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependency::Runtime, "Model".into())]
        );
    }
}
