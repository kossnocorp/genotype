use crate::prelude::internal::*;

impl TSConvert<TSProperty> for GTProperty {
    fn convert(&self, context: &mut TSConvertContext) -> TSProperty {
        let descriptor = self.descriptor.convert(context);

        let descriptor = if self.required {
            descriptor
        } else {
            TSUnion {
                descriptors: vec![descriptor, TSPrimitive::Undefined.into()],
            }
            .into()
        };

        TSProperty {
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
            GTProperty {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: Gt::primitive_string().into(),
                required: true,
            }
            .convert(&mut Default::default()),
            @r#"
        TSProperty(
          doc: None,
          name: TSKey("name"),
          descriptor: Primitive(String),
          required: true,
        )
        "#
        );
    }

    #[test]
    fn test_convert_doc() {
        assert_ron_snapshot!(
            GTProperty {
                span: (0, 0).into(),
                doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: Gt::primitive_string().into(),
                required: true,
            }
            .convert(&mut Default::default()),
            @r#"
        TSProperty(
          doc: Some(TSDoc("Hello, world!")),
          name: TSKey("name"),
          descriptor: Primitive(String),
          required: true,
        )
        "#
        );
    }

    #[test]
    fn test_convert_optional() {
        assert_ron_snapshot!(
            GTProperty {
                span: (0, 0).into(),
                doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: Gt::primitive_string().into(),
                required: false,
            }
            .convert(&mut Default::default()),
            @r#"
        TSProperty(
          doc: Some(TSDoc("Hello, world!")),
          name: TSKey("name"),
          descriptor: Union(TSUnion(
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
