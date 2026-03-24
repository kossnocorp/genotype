use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaTuple> for GtjTuple {
    fn to_schema(&self) -> GtjSchemaTuple {
        GtjSchemaTuple {
            r#type: GtjSchemaTupleTypeArray,
            title: self.name.clone(),
            description: self.doc.clone(),
            prefix_items: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.to_schema())
                .collect(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjTuple {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaTuple(self.to_schema())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let tuple = GtjTuple {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjTupleTypeTuple,
            descriptors: vec![GtjAny::GtjBoolean(GtjBoolean {
                name: None,
                doc: None,
                r#type: GtjBooleanTypeBoolean,
            })],
        };

        let any_schema: GtjSchemaAny = tuple.to_schema();
        assert_ron_snapshot!(any_schema, @r#"
        GtjSchemaTuple(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "array",
          prefixItems: [
            GtjSchemaBoolean(
              type: "boolean",
            ),
          ],
        )
        "#);

        let tuple_schema: GtjSchemaTuple = tuple.to_schema();
        assert_ron_snapshot!(tuple_schema, @r#"
        GtjSchemaTuple(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "array",
          prefixItems: [
            GtjSchemaBoolean(
              type: "boolean",
            ),
          ],
        )
        "#);
    }
}
