use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaBoolean> for GtjBoolean {
    fn to_schema(&self) -> GtjSchemaBoolean {
        GtjSchemaBoolean {
            r#type: GtjSchemaBooleanTypeBoolean,
            title: self.name.clone(),
            description: self.doc.clone(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjBoolean {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaBoolean(self.to_schema())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let boolean = GtjBoolean {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjBooleanTypeBoolean,
        };

        let any_schema: GtjSchemaAny = boolean.to_schema();
        assert_ron_snapshot!(any_schema, @r#"
        GtjSchemaBoolean(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "boolean",
        )
        "#);

        let boolean_schema: GtjSchemaBoolean = boolean.to_schema();
        assert_ron_snapshot!(boolean_schema, @r#"
        GtjSchemaBoolean(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "boolean",
        )
        "#);
    }
}
