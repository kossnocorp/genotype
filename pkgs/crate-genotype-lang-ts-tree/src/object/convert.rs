use crate::prelude::internal::*;

impl TsConvert<TsObject> for GtObject {
    fn convert(&self, context: &mut TsConvertContext) -> TsObject {
        TsObject {
            properties: self
                .properties
                .iter()
                .map(|property| property.convert(context))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_node(Gt::object("Person", vec![
                Gt::property("name", Gt::primitive_string()),
                Gt::property_optional("age", Gt::primitive_i32()),
            ])),
            @r#"
        TsObject(
          properties: [
            TsProperty(
              doc: None,
              name: TsKey("name"),
              descriptor: Primitive(String),
              required: true,
            ),
            TsProperty(
              doc: None,
              name: TsKey("age"),
              descriptor: Union(TsUnion(
                descriptors: [
                  Primitive(Number),
                  Primitive(Undefined),
                ],
              )),
              required: false,
            ),
          ],
        )
        "#
        );
    }
}
