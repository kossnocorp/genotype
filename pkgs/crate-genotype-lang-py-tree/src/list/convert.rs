use crate::prelude::internal::*;

impl PyConvert<PyList> for GtArray {
    fn convert(&self, context: &mut PyConvertContext) -> PyList {
        PyList {
            descriptor: self.descriptor.convert(context),
        }
        .resolve(context)
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
            convert_node(Gt::array(Gt::primitive_boolean())),
            @"
        PyList(
          descriptor: Primitive(Boolean),
        )
        "
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PyConvertContext::new(
            Default::default(),
            PyConfig {
                lang: PyConfigLang::new(PyVersion::Legacy),
                ..Default::default()
            },
        );
        assert_ron_snapshot!(
            Gt::array(Gt::primitive_string())
            .convert(&mut context),
            @"
        PyList(
          descriptor: Primitive(String),
        )
        "
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Typing, PyIdentifier("List")),
        ]
        "#
        );
    }
}
