use crate::prelude::internal::*;

impl TsConvert<TsProperty> for GtProperty {
    fn convert(&self, context: &mut TsConvertContext) -> TsProperty {
        let descriptor = self.descriptor.convert(context);

        let descriptor = if self.required {
            descriptor
        } else {
            TsUnion {
                descriptors: vec![descriptor, TsPrimitive::Undefined.into()],
            }
            .into()
        };

        TsProperty {
            doc: self.doc.as_ref().map(|d| d.convert(context)),
            name: self.name.convert(context),
            descriptor,
            required: self.required,
        }
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
                required: true,
            }
            .convert(&mut Default::default()),
            @r#"
        TsProperty(
          doc: None,
          name: TsKey("name"),
          descriptor: Primitive(String),
          required: true,
        )
        "#
        );
    }

    #[test]
    fn test_convert_doc() {
        assert_ron_snapshot!(
            GtProperty {
                span: (0, 0).into(),
                doc: Some(GtDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GtKey::new((0, 0).into(), "name".into()),
                descriptor: Gt::primitive_string().into(),
                required: true,
            }
            .convert(&mut Default::default()),
            @r#"
        TsProperty(
          doc: Some(TsDoc("Hello, world!")),
          name: TsKey("name"),
          descriptor: Primitive(String),
          required: true,
        )
        "#
        );
    }

    #[test]
    fn test_convert_optional() {
        assert_ron_snapshot!(
            GtProperty {
                span: (0, 0).into(),
                doc: Some(GtDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GtKey::new((0, 0).into(), "name".into()),
                descriptor: Gt::primitive_string().into(),
                required: false,
            }
            .convert(&mut Default::default()),
            @r#"
        TsProperty(
          doc: Some(TsDoc("Hello, world!")),
          name: TsKey("name"),
          descriptor: Union(TsUnion(
            descriptors: [
              Primitive(String),
              Primitive(Undefined),
            ],
          )),
          required: false,
        )
        "#
        );
    }
}
