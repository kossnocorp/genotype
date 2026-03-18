use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaNull> for GtjNull {
    fn to_schema(&self) -> GtjSchemaNull {
        GtjSchemaNull {
            r#type: GtjSchemaNullTypeNull,
            title: self.name.clone(),
            description: self.doc.clone(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjNull {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaNull(self.to_schema())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let null = GtjNull {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjNullTypeNull,
        };

        let any_schema: GtjSchemaAny = null.to_schema();
        assert_ron_snapshot!(any_schema, @r#"
        GtjSchemaNull(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "null",
        )
        "#);

        let null_schema: GtjSchemaNull = null.to_schema();
        assert_ron_snapshot!(null_schema, @r#"
        GtjSchemaNull(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "null",
        )
        "#);
    }
}
