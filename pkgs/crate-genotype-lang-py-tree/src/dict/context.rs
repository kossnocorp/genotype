use crate::prelude::internal::*;

impl PyContextResolve for PyDict {
    fn resolve(self, context: &mut PyConvertContext) -> Self {
        if context.is_version(PyVersion::Legacy) {
            context.push_import(PyImport::new(PyDependencyIdent::Typing, "Dict".into()));
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
        let dict = PyDict {
            key: PyDictKey::String,
            descriptor: PyPrimitive::String.into(),
        };
        dict.resolve(&mut context);
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
        let dict = PyDict {
            key: PyDictKey::String,
            descriptor: PyPrimitive::String.into(),
        };
        dict.resolve(&mut context);
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Typing,
            reference: Named([
              Name(PyIdentifier("Dict")),
            ]),
          ),
        ]
        "#
        );
    }
}
