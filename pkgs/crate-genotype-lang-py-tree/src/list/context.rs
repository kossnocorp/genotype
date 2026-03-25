use crate::prelude::internal::*;

impl PyContextResolve for PyList {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PyConvertContextConstraint,
    {
        if context.is_version(PyVersion::Legacy) {
            context.add_import(PyDependencyIdent::Typing, "List".into());
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
        let list = PyList {
            descriptor: PyPrimitive::String.into(),
        };
        list.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PyConvertContextMock::new(PyVersion::Legacy);
        let list = PyList {
            descriptor: PyPrimitive::String.into(),
        };
        list.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PyDependencyIdent::Typing, "List".into())]
        );
    }
}
