use crate::prelude::internal::*;

impl PYConvert<PYProperty> for GTProperty {
    fn convert(&self, context: &mut PYConvertContext) -> PYProperty {
        PYProperty {
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
            GTProperty {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: Gt::primitive_string().into(),
                required: false,
            }
            .convert(&mut PYConvertContext::default()),
            @r#"
        PYProperty(
          doc: None,
          name: PYKey("name"),
          descriptor: Primitive(String),
          required: false,
        )
        "#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::new(
            Default::default(),
            PyConfig {
                lang: PyConfigLang::new(PYVersion::Legacy),
                ..Default::default()
            },
        );
        assert_ron_snapshot!(
            GTProperty {
                doc: None,
                span: (0, 0).into(),
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: Gt::primitive_string().into(),
                required: false,
            }
            .convert(&mut context),
            @r#"
        PYProperty(
          doc: None,
          name: PYKey("name"),
          descriptor: Primitive(String),
          required: false,
        )
        "#
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Typing, PYIdentifier("Optional")),
        ]
        "#
        );
    }
}
