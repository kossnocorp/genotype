use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaLiteral> for GtjLiteral {
    fn to_schema(&self) -> GtjSchemaLiteral {
        GtjSchemaLiteral {
            title: self.name.clone(),
            description: self.doc.clone(),
            r#const: match self.value {
                GtjLiteralValue::Null => GtjSchemaLiteralConst::Null,
                GtjLiteralValue::Boolean(value) => GtjSchemaLiteralConst::Boolean(value),
                GtjLiteralValue::Number(value) => GtjSchemaLiteralConst::Number(value),
                GtjLiteralValue::String(ref value) => GtjSchemaLiteralConst::String(value.clone()),
            },
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjLiteral {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaLiteral(self.to_schema())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_null() {
        let literal = GtjLiteral {
            r#type: GtjLiteralTypeLiteral,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            value: GtjLiteralValue::Null,
        };

        let any_schema: GtjSchemaAny = literal.to_schema();
        assert_ron_snapshot!(any_schema, @r#"
        GtjSchemaLiteral(
          title: Some("hello"),
          description: Some("Hello, world!"),
          const: (),
        )
        "#);

        let literal_schema: GtjSchemaLiteral = literal.to_schema();
        assert_ron_snapshot!(literal_schema, @r#"
        GtjSchemaLiteral(
          title: Some("hello"),
          description: Some("Hello, world!"),
          const: (),
        )
        "#);
    }

    #[test]
    fn test_convert_boolean() {
        let literal = GtjLiteral {
            r#type: GtjLiteralTypeLiteral,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            value: GtjLiteralValue::Boolean(true),
        };

        let any_schema: GtjSchemaAny = literal.to_schema();
        assert_ron_snapshot!(any_schema, @r#"
        GtjSchemaLiteral(
          title: Some("hello"),
          description: Some("Hello, world!"),
          const: true,
        )
        "#);

        let literal_schema: GtjSchemaLiteral = literal.to_schema();
        assert_ron_snapshot!(literal_schema, @r#"
        GtjSchemaLiteral(
          title: Some("hello"),
          description: Some("Hello, world!"),
          const: true,
        )
        "#);
    }

    #[test]
    fn test_convert_number() {
        let literal = GtjLiteral {
            r#type: GtjLiteralTypeLiteral,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            value: GtjLiteralValue::Number(42.0),
        };

        let any_schema: GtjSchemaAny = literal.to_schema();
        assert_ron_snapshot!(any_schema, @r#"
        GtjSchemaLiteral(
          title: Some("hello"),
          description: Some("Hello, world!"),
          const: 42.0,
        )
        "#);

        let literal_schema: GtjSchemaLiteral = literal.to_schema();
        assert_ron_snapshot!(literal_schema, @r#"
        GtjSchemaLiteral(
          title: Some("hello"),
          description: Some("Hello, world!"),
          const: 42.0,
        )
        "#);
    }

    #[test]
    fn test_convert_string() {
        let literal = GtjLiteral {
            r#type: GtjLiteralTypeLiteral,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            value: GtjLiteralValue::String("Hello".into()),
        };

        let any_schema: GtjSchemaAny = literal.to_schema();
        assert_ron_snapshot!(any_schema, @r#"
        GtjSchemaLiteral(
          title: Some("hello"),
          description: Some("Hello, world!"),
          const: "Hello",
        )
        "#);

        let literal_schema: GtjSchemaLiteral = literal.to_schema();
        assert_ron_snapshot!(literal_schema, @r#"
        GtjSchemaLiteral(
          title: Some("hello"),
          description: Some("Hello, world!"),
          const: "Hello",
        )
        "#);
    }
}
