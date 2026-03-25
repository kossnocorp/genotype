use crate::prelude::internal::*;

impl PyContextResolve for PyUnion {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PyConvertContextConstraint,
    {
        if context.is_version(PyVersion::Legacy) {
            context.add_import(PyDependencyIdent::Typing, "Union".into());
        }

        if self.discriminator.is_some() {
            if context.is_version(PyVersion::Legacy) {
                context.add_import(PyDependencyIdent::TypingExtensions, "Annotated".into());
            } else {
                context.add_import(PyDependencyIdent::Typing, "Annotated".into());
            }

            context.add_import(PyDependencyIdent::Pydantic, "Field".into());
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
        let union = PyUnion {
            descriptors: vec![PyPrimitive::String.into()],
            discriminator: None,
        };
        union.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PyConvertContextMock::new(PyVersion::Legacy);
        let union = PyUnion {
            descriptors: vec![PyPrimitive::String.into()],
            discriminator: None,
        };
        union.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PyDependencyIdent::Typing, "Union".into())]
        );
    }

    #[test]
    fn test_resolve_discriminator() {
        let mut context = PyConvertContextMock::default();
        let union = PyUnion {
            descriptors: vec![PyPrimitive::String.into()],
            discriminator: Some("type".into()),
        };
        union.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![
                (PyDependencyIdent::Typing, "Annotated".into()),
                (PyDependencyIdent::Pydantic, "Field".into())
            ]
        );
    }

    #[test]
    fn test_resolve_discriminator_legacy() {
        let mut context = PyConvertContextMock::new(PyVersion::Legacy);
        let union = PyUnion {
            descriptors: vec![PyPrimitive::String.into()],
            discriminator: Some("type".into()),
        };
        union.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![
                (PyDependencyIdent::Typing, "Union".into()),
                (PyDependencyIdent::TypingExtensions, "Annotated".into()),
                (PyDependencyIdent::Pydantic, "Field".into())
            ]
        );
    }
}
