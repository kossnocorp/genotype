use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaArray> for GtjArray {
    fn to_schema(&self) -> GtjSchemaArray {
        GtjSchemaArray {
            r#type: GtjSchemaArrayTypeArray,
            title: self.name.clone(),
            description: self.doc.clone(),
            items: self.descriptor.to_schema(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjArray {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaArray(Box::new(self.to_schema()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let array = GtjArray {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjBoolean(GtjBoolean {
                name: None,
                doc: None,
                r#type: GtjBooleanTypeBoolean,
            }),
        };

        let any_schema: GtjSchemaAny = array.to_schema();
        assert_ron_snapshot!(any_schema, @r#"
        GtjSchemaArray(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "array",
          items: GtjSchemaBoolean(
            type: "boolean",
          ),
        )
        "#);

        let array_schema: GtjSchemaAny = array.to_schema();
        assert_ron_snapshot!(array_schema, @r#"
        GtjSchemaArray(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "array",
          items: GtjSchemaBoolean(
            type: "boolean",
          ),
        )
        "#);
    }
}
