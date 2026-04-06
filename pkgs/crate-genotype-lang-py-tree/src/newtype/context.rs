use crate::prelude::internal::*;

impl PyContextResolve for PyNewtype {
    fn resolve(self, context: &mut PyConvertContext) -> Self {
        context.push_import(PyImport::new(PyDependencyIdent::Typing, "NewType".into()));
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
        let alias = PyNewtype {
            doc: None,
            name: "Foo".into(),
            primitive: PyPrimitive::String,
        };
        alias.resolve(&mut context);
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Typing,
            reference: Named([
              Name(PyIdentifier("NewType")),
            ]),
          ),
        ]
        "#
        );
    }
}
