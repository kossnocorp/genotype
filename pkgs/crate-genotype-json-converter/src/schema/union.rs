use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaUnion> for GtjUnion {
    fn to_schema(&self) -> GtjSchemaUnion {
        GtjSchemaUnion {
            title: self.name.clone(),
            description: self.doc.clone(),
            any_of: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.to_schema())
                .collect(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjUnion {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaUnion(self.to_schema())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let union = GtjUnion {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjUnionTypeUnion,
            descriptors: vec![GtjAny::GtjBoolean(GtjBoolean {
                name: None,
                doc: None,
                r#type: GtjBooleanTypeBoolean,
            })],
        };

        let any_schema: GtjSchemaAny = union.to_schema();
        assert_ron_snapshot!(any_schema, @r#"
        GtjSchemaUnion(
          title: Some("hello"),
          description: Some("Hello, world!"),
          anyOf: [
            GtjSchemaBoolean(
              type: "boolean",
            ),
          ],
        )
        "#);

        let union_schema: GtjSchemaUnion = union.to_schema();
        assert_ron_snapshot!(union_schema, @r#"
        GtjSchemaUnion(
          title: Some("hello"),
          description: Some("Hello, world!"),
          anyOf: [
            GtjSchemaBoolean(
              type: "boolean",
            ),
          ],
        )
        "#);
    }
}
