use crate::*;

use super::RSField;

impl RSContextResolve for RSField {
    fn resolve<Context>(self, _context: &mut Context) -> Self
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
        let alias = RSField {
            doc: None,
            attributes: vec![],
            name: "foo".into(),
            descriptor: RSPrimitive::String.into(),
        };
        alias.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }
}
