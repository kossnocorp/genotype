use crate::prelude::internal::*;

impl PyContextResolve for PyList {
    fn resolve(self, context: &mut PyConvertContext) -> Self {
        if context.is_version(PyVersion::Legacy) {
            context.push_import(PyImport::new(PyDependencyIdent::Typing, "List".into()));
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
        let list = PyList {
            descriptor: PyPrimitive::String.into(),
        };
        list.resolve(&mut context);
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
        let list = PyList {
            descriptor: PyPrimitive::String.into(),
        };
        list.resolve(&mut context);
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Typing,
            reference: Named([
              Name(PyIdentifier("List")),
            ]),
          ),
        ]
        "#
        );
    }
}
