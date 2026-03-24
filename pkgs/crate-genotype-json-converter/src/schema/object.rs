use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaObject> for GtjObject {
    fn to_schema(&self) -> GtjSchemaObject {
        let mut required = vec![];
        let properties = self
            .properties
            .iter()
            .map(|property| {
                let name = property.name.clone();
                let schema = property.descriptor.to_schema();
                if property.required {
                    required.push(name.clone());
                }
                (name, schema)
            })
            .collect();
        GtjSchemaObject {
            r#type: GtjSchemaObjectTypeObject,
            title: self.name.clone(),
            description: self.doc.clone(),
            properties,
            required: Some(required),
            additional_properties: Some(false),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjObject {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaObject(self.to_schema())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let object = GtjObject {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjObjectTypeObject,
            properties: vec![{
                GtjProperty {
                    r#type: GtjPropertyTypeProperty,
                    name: "foo".into(),
                    doc: None,
                    descriptor: GtjAny::GtjBoolean(GtjBoolean {
                        name: None,
                        doc: None,
                        r#type: GtjBooleanTypeBoolean,
                    }),
                    required: true,
                }
            }],
        };

        let any_schema: GtjSchemaAny = object.to_schema();
        assert_ron_snapshot!(any_schema, @r#"
        GtjSchemaObject(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "object",
          properties: {
            "foo": GtjSchemaBoolean(
              type: "boolean",
            ),
          },
          required: Some([
            "foo",
          ]),
          additionalProperties: Some(false),
        )
        "#);

        let object_schema: GtjSchemaObject = object.to_schema();
        assert_ron_snapshot!(object_schema, @r#"
        GtjSchemaObject(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "object",
          properties: {
            "foo": GtjSchemaBoolean(
              type: "boolean",
            ),
          },
          required: Some([
            "foo",
          ]),
          additionalProperties: Some(false),
        )
        "#);
    }
}
