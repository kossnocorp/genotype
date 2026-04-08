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
            convert_node(Gt::property("name", Gt::primitive_string())),
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
            convert_node(assign!(
                Gt::property("name", Gt::primitive_string()),
                doc = Gt::some_doc("Hello, world!")
            )),
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
            convert_node(assign!(
                Gt::property_optional("name", Gt::primitive_string()),
                doc = Gt::some_doc("Hello, world!")
            )),
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
