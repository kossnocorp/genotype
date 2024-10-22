use genotype_lang_py_config::PYVersion;

use crate::*;

use super::PYUnion;

impl PYContextResolve for PYUnion {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYContext,
    {
        if context.is_version(PYVersion::Legacy) {
            context.import(PYDependency::Typing, "Union".into());
        }

        if self.discriminator.is_some() {
            if context.is_version(PYVersion::Legacy) {
                context.import(PYDependency::TypingExtensions, "Annotated".into());
            } else {
                context.import(PYDependency::Typing, "Annotated".into());
            }

            context.import(PYDependency::Pydantic, "Field".into());
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
        let union = PYUnion {
            descriptors: vec![PYPrimitive::String.into()],
            discriminator: None,
        };
        union.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYContextMock::new(PYVersion::Legacy);
        let union = PYUnion {
            descriptors: vec![PYPrimitive::String.into()],
            discriminator: None,
        };
        union.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![(PYDependency::Typing, "Union".into())]
        );
    }

    #[test]
    fn test_resolve_discriminator() {
        let mut context = PYContextMock::default();
        let union = PYUnion {
            descriptors: vec![PYPrimitive::String.into()],
            discriminator: Some("type".into()),
        };
        union.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![
                (PYDependency::Typing, "Annotated".into()),
                (PYDependency::Pydantic, "Field".into())
            ]
        );
    }

    #[test]
    fn test_resolve_discriminator_legacy() {
        let mut context = PYContextMock::new(PYVersion::Legacy);
        let union = PYUnion {
            descriptors: vec![PYPrimitive::String.into()],
            discriminator: Some("type".into()),
        };
        union.resolve(&mut context);
        assert_eq!(
            context.as_imports(),
            vec![
                (PYDependency::Typing, "Union".into()),
                (PYDependency::TypingExtensions, "Annotated".into()),
                (PYDependency::Pydantic, "Field".into())
            ]
        );
    }
}
