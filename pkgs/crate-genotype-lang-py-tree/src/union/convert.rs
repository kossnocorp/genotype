use crate::prelude::internal::*;

impl PyConvert<PyUnion> for GtUnion {
    fn convert(&self, context: &mut PyConvertContext) -> PyUnion {
        PyUnion {
            descriptors: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(context))
                .collect(),
            discriminator: None,
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
            convert_node(Gt::union(vec![
                Gt::primitive_boolean().into(),
                Gt::primitive_string().into(),
            ])),
            @"
        PyUnion(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
          discriminator: None,
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
            convert_node_with(Gt::union(vec![
                Gt::primitive_string().into(),
            ]), &mut context),
            @"
        PyUnion(
          descriptors: [
            Primitive(String),
          ],
          discriminator: None,
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
              Name(PyIdentifier("Union")),
            ]),
          ),
        ]
        "#
        );
    }
}
