use crate::*;

use super::RSHashMap;

impl RSContextResolve for RSHashMap {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: RSContext,
    {
        context.import(RSDependency::Std("collections".into()), "HashMap".into());
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
        let dict = RSHashMap {
            key: RSPrimitive::String.into(),
            descriptor: RSPrimitive::String.into(),
        };
        dict.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(RSDependency::Std("collections".into()), "HashMap".into())]
        );
    }
}
