use genotype_lang_py_config::PYVersion;

use crate::*;

use super::PYTuple;

impl PYContextResolve for PYTuple {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYContext,
    {
        if context.is_version(PYVersion::Legacy) {
            context.import(PYDependency::Typing, "Tuple".into());
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use genotype_lang_py_config::PYVersion;
    use mock::PYContextMock;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_resolve() {
        let mut context = PYContextMock::default();
        let tuple = PYTuple {
            descriptors: vec![PYPrimitive::String.into()],
        };
        tuple.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYContextMock::new(PYVersion::Legacy);
        let tuple = PYTuple {
            descriptors: vec![PYPrimitive::String.into()],
        };
        tuple.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependency::Typing, "Tuple".into())],
        );
    }
}
