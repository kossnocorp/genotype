use crate::prelude::internal::*;

impl PyContextResolve for PyDict {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PyConvertContextConstraint,
    {
        if context.is_version(PyVersion::Legacy) {
            context.add_import(PyDependencyIdent::Typing, "Dict".into());
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_resolve() {
        let mut context = PyConvertContextMock::default();
        let dict = PyDict {
            key: PyDictKey::String,
            descriptor: PyPrimitive::String.into(),
        };
        dict.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PyConvertContextMock::new(PyVersion::Legacy);
        let dict = PyDict {
            key: PyDictKey::String,
            descriptor: PyPrimitive::String.into(),
        };
        dict.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PyDependencyIdent::Typing, "Dict".into())],
        );
    }
}
