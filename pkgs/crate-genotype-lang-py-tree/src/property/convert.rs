use crate::prelude::internal::*;

impl PyConvert<PyProperty> for GtProperty {
    fn convert(&self, context: &mut PyConvertContext) -> PyProperty {
        let name = self.name.convert(context);
        let original_name = self.name.1.as_ref();
        let alias = if name.0.as_ref() == original_name {
            None
        } else {
            Some(original_name.to_string())
        };

        PyProperty {
            doc: self.doc.as_ref().map(|doc| doc.convert(context)),
            name,
            alias,
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
          alias: None,
          descriptor: Primitive(String),
          required: false,
        )
        "#
        );
    }

    #[test]
    fn test_convert_alias() {
        assert_ron_snapshot!(
            GtProperty {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtKey::new((0, 0).into(), "filePath".into()),
                descriptor: Gt::primitive_string().into(),
                required: true,
            }
            .convert(&mut PyConvertContext::default()),
            @r#"
        PyProperty(
          doc: None,
          name: PyKey("file_path"),
          alias: Some("filePath"),
          descriptor: Primitive(String),
          required: true,
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
          alias: None,
          descriptor: Primitive(String),
          required: false,
        )
        "#
        );
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          PyImport(
            dependency: Typing,
            reference: Named([
              Name(PyIdentifier("Optional")),
            ]),
          ),
        ]
        "#
        );
    }
}
