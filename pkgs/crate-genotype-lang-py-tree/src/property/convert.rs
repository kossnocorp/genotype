use crate::prelude::internal::*;

impl PyConvert<PyProperty> for GtProperty {
    fn convert(&self, context: &mut PyConvertContext) -> PyProperty {
        PyProperty {
            doc: self.doc.as_ref().and_then(|doc| Some(doc.convert(context))),
            name: self.name.convert(context),
            descriptor: self.descriptor.convert(context),
            required: self.required,
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
            GtProperty {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtKey::new((0, 0).into(), "name".into()),
                descriptor: Gt::primitive_string().into(),
                required: false,
            }
            .convert(&mut PyConvertContext::default()),
            @r#"
        PyProperty(
          doc: None,
          name: PyKey("name"),
          descriptor: Primitive(String),
          required: false,
        )
        "#
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
            GtProperty {
                doc: None,
                span: (0, 0).into(),
                attributes: vec![],
                name: GtKey::new((0, 0).into(), "name".into()),
                descriptor: Gt::primitive_string().into(),
                required: false,
            }
            .convert(&mut context),
            @r#"
        PyProperty(
          doc: None,
          name: PyKey("name"),
          descriptor: Primitive(String),
          required: false,
        )
        "#
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Typing, PyIdentifier("Optional")),
        ]
        "#
        );
    }
}
