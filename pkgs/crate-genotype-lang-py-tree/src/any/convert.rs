use crate::prelude::internal::*;

impl PyConvert<PyAny> for GtAny {
    fn convert(&self, resolve: &mut PyConvertContext) -> PyAny {
        PyAny.resolve(resolve)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_node(Gt::any()),
            @"PyAny"
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PyConvertContext::default();
        assert_ron_snapshot!(
            convert_node_with(Gt::any(), &mut context),
            @"PyAny"
        );

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
