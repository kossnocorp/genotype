use crate::prelude::internal::*;

impl PyContextResolve for PyTuple {
    fn resolve(self, context: &mut PyConvertContext) -> Self {
        if context.is_version(PyVersion::Legacy) {
            context.push_import(PyImport::new(PyDependencyIdent::Typing, "Tuple".into()));
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_resolve() {
        let mut context = PyConvertContext::default();
        let tuple = PyTuple {
            descriptors: vec![PyPrimitive::String.into()],
        };
        tuple.resolve(&mut context);
        assert_ron_snapshot!(context.imports(), @"[]");
    }

    #[test]
    fn test_resolve_legacy() {
        let mut config = PyConfig::default();
        config.lang.version = PyVersion::Legacy;
        let mut context = PyConvertContext::new(Default::default(), config);
        let tuple = PyTuple {
            descriptors: vec![PyPrimitive::String.into()],
        };
        tuple.resolve(&mut context);
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Typing,
            reference: Named([
              Name(PyIdentifier("Tuple")),
            ]),
          ),
        ]
        "#
        );
    }
}
