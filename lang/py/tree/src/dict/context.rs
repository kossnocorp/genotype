use genotype_lang_py_config::PYVersion;

use crate::*;

use super::PYDict;

impl PYContextResolve for PYDict {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYContext,
    {
        if context.is_version(PYVersion::Legacy) {
            context.import(PYDependency::Typing, "Dict".into());
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
        let dict = PYDict {
            key: PYDictKey::String,
            descriptor: PYPrimitive::String.into(),
        };
        dict.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYContextMock::new(PYVersion::Legacy);
        let dict = PYDict {
            key: PYDictKey::String,
            descriptor: PYPrimitive::String.into(),
        };
        dict.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependency::Typing, "Dict".into())],
        );
    }
}
