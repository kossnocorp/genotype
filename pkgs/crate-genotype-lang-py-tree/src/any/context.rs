use crate::prelude::internal::*;

impl PyContextResolve for PyAny {
    fn resolve(self, context: &mut PyConvertContext) -> Self {
        context.push_import(PyImport::new(PyDependencyIdent::Typing, "Any".into()));
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
        let alias = PyAny;
        alias.resolve(&mut context);
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Typing,
            reference: Named([
              Name(PyIdentifier("Any")),
            ]),
          ),
        ]
        "#
        );
    }
}
