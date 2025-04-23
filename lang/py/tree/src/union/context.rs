use crate::prelude::internal::*;

impl PYContextResolve for PYUnion {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYConvertContextConstraint,
    {
        if context.is_version(PYVersion::Legacy) {
            context.add_import(PYDependencyIdent::Typing, "Union".into());
        }

        if self.discriminator.is_some() {
            if context.is_version(PYVersion::Legacy) {
                context.add_import(PYDependencyIdent::TypingExtensions, "Annotated".into());
            } else {
                context.add_import(PYDependencyIdent::Typing, "Annotated".into());
            }

            context.add_import(PYDependencyIdent::Pydantic, "Field".into());
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
        let mut context = PYConvertContextMock::default();
        let union = PYUnion {
            descriptors: vec![PYPrimitive::String.into()],
            discriminator: None,
        };
        union.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYConvertContextMock::new(PYVersion::Legacy);
        let union = PYUnion {
            descriptors: vec![PYPrimitive::String.into()],
            discriminator: None,
        };
        union.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependencyIdent::Typing, "Union".into())]
        );
    }

    #[test]
    fn test_resolve_discriminator() {
        let mut context = PYConvertContextMock::default();
        let union = PYUnion {
            descriptors: vec![PYPrimitive::String.into()],
            discriminator: Some("type".into()),
        };
        union.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![
                (PYDependencyIdent::Typing, "Annotated".into()),
                (PYDependencyIdent::Pydantic, "Field".into())
            ]
        );
    }

    #[test]
    fn test_resolve_discriminator_legacy() {
        let mut context = PYConvertContextMock::new(PYVersion::Legacy);
        let union = PYUnion {
            descriptors: vec![PYPrimitive::String.into()],
            discriminator: Some("type".into()),
        };
        union.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![
                (PYDependencyIdent::Typing, "Union".into()),
                (PYDependencyIdent::TypingExtensions, "Annotated".into()),
                (PYDependencyIdent::Pydantic, "Field".into())
            ]
        );
    }
}
