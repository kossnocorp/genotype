use crate::prelude::internal::*;

impl PyContextResolve for PyUnion {
    fn resolve(self, context: &mut PyConvertContext) -> Self {
        if context.is_version(PyVersion::Legacy) {
            context.push_import(PyImport::new(PyDependencyIdent::Typing, "Union".into()));
        }

        if self.discriminator.is_some() {
            if context.is_version(PyVersion::Legacy) {
                context.push_import(PyImport::new(
                    PyDependencyIdent::TypingExtensions,
                    "Annotated".into(),
                ));
            } else {
                context.push_import(PyImport::new(PyDependencyIdent::Typing, "Annotated".into()));
            }

            context.push_import(PyImport::new(PyDependencyIdent::Pydantic, "Field".into()));
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_resolve() {
        let mut context = PyConvertContext::default();
        let union = PyUnion {
            descriptors: vec![PyPrimitive::String.into()],
            discriminator: None,
        };
        union.resolve(&mut context);
        assert_ron_snapshot!(
            context.imports(),
            @"[]"
        );
    }

    #[test]
    fn test_resolve_legacy() {
        let mut config = PyConfig::default();
        config.lang.version = PyVersion::Legacy;
        let mut context = PyConvertContext::new(Default::default(), config);
        let union = PyUnion {
            descriptors: vec![PyPrimitive::String.into()],
            discriminator: None,
        };
        union.resolve(&mut context);
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Typing,
            reference: Named([
              Name(PyIdentifier("Union")),
            ]),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_resolve_discriminator() {
        let mut context = PyConvertContext::default();
        let union = PyUnion {
            descriptors: vec![PyPrimitive::String.into()],
            discriminator: Some("type".into()),
        };
        union.resolve(&mut context);
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Typing,
            reference: Named([
              Name(PyIdentifier("Annotated")),
            ]),
          ),
          PyImport(
            dependency: Pydantic,
            reference: Named([
              Name(PyIdentifier("Field")),
            ]),
          ),
        ]
        "#
        );
    }

    #[test]
    fn test_resolve_discriminator_legacy() {
        let mut config = PyConfig::default();
        config.lang.version = PyVersion::Legacy;
        let mut context = PyConvertContext::new(Default::default(), config);
        let union = PyUnion {
            descriptors: vec![PyPrimitive::String.into()],
            discriminator: Some("type".into()),
        };
        union.resolve(&mut context);
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Typing,
            reference: Named([
              Name(PyIdentifier("Union")),
            ]),
          ),
          PyImport(
            dependency: TypingExtensions,
            reference: Named([
              Name(PyIdentifier("Annotated")),
            ]),
          ),
          PyImport(
            dependency: Pydantic,
            reference: Named([
              Name(PyIdentifier("Field")),
            ]),
          ),
        ]
        "#
        );
    }
}
