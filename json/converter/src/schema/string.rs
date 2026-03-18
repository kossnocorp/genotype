use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaString> for GtjString {
    fn to_schema(&self) -> GtjSchemaString {
        GtjSchemaString {
            r#type: GtjSchemaStringTypeString,
            title: self.name.clone(),
            description: self.doc.clone(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjString {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaString(self.to_schema())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let string = GtjString {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjStringTypeString,
        };

        let any_schema: GtjSchemaAny = string.to_schema();
        assert_ron_snapshot!(any_schema, @r#"
        GtjSchemaString(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "string",
        )
        "#);

        let string_schema: GtjSchemaString = string.to_schema();
        assert_ron_snapshot!(string_schema, @r#"
        GtjSchemaString(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "string",
        )
        "#);
    }
}
