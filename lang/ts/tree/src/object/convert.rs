use crate::prelude::internal::*;

impl TSConvert<TSObject> for GTObject {
    fn convert(&self, context: &mut TSConvertContext) -> TSObject {
        TSObject {
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
        TSObject(
          properties: [
            TSProperty(
              doc: None,
              name: TSKey("name"),
              descriptor: Primitive(String),
              required: true,
            ),
            TSProperty(
              doc: None,
              name: TSKey("age"),
              descriptor: Union(TSUnion(
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
