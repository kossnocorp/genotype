use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaNumber> for GtjNumber {
    fn to_schema(&self) -> GtjSchemaNumber {
        GtjSchemaNumber {
            r#type: GtjSchemaNumberTypeNumber,
            title: self.name.clone(),
            description: self.doc.clone(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjNumber {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaNumber(self.to_schema())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let number = GtjNumber {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjNumberTypeNumber,
        };

        let any_schema: GtjSchemaAny = number.to_schema();
        assert_ron_snapshot!(any_schema, @r#"
        GtjSchemaNumber(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "number",
        )
        "#);

        let number_schema: GtjSchemaNumber = number.to_schema();
        assert_ron_snapshot!(number_schema, @r#"
        GtjSchemaNumber(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "number",
        )
        "#);
    }
}
