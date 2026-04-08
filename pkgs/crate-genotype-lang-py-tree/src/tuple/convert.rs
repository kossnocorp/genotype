use crate::prelude::internal::*;

impl PyConvert<PyTuple> for GtTuple {
    fn convert(&self, context: &mut PyConvertContext) -> PyTuple {
        PyTuple {
            descriptors: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(context))
                .collect(),
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_node(Gt::tuple(vec![
                Gt::primitive_boolean().into(),
                Gt::primitive_string().into(),
            ])),
            @"
        PyTuple(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
        )
        "
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PyConvertContext::new(
            PyConvertResolve::default(),
            PyConfig {
                lang: PyConfigLang::new(PyVersion::Legacy),
                ..Default::default()
            },
        );
        assert_ron_snapshot!(
            convert_node_with(Gt::tuple(vec![
                Gt::primitive_string().into(),
            ]), &mut context),
            @"
        PyTuple(
          descriptors: [
            Primitive(String),
          ],
        )
        "
        );
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
