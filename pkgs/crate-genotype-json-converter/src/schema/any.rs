use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaAny> for GtjAny {
    fn to_schema(&self) -> GtjSchemaAny {
        match self {
            GtjAny::GtjBoolean(boolean) => boolean.to_schema(),
            GtjAny::GtjNumber(number) => number.to_schema(),
            GtjAny::GtjString(string) => string.to_schema(),
            GtjAny::GtjObject(object) => object.to_schema(),
            GtjAny::GtjArray(array) => array.to_schema(),
            GtjAny::GtjUnion(union) => union.to_schema(),
            GtjAny::GtjTuple(tuple) => tuple.to_schema(),
            GtjAny::GtjLiteral(literal) => literal.to_schema(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_boolean() {
        let boolean = GtjAny::GtjBoolean(GtjBoolean {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjBooleanTypeBoolean,
        });
        assert_ron_snapshot!(boolean.to_schema(), @r#"
        GtjSchemaBoolean(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "boolean",
        )
        "#);
    }

    #[test]
    fn test_convert_number() {
        let number = GtjAny::GtjNumber(GtjNumber {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjNumberTypeNumber,
        });
        assert_ron_snapshot!(number.to_schema(), @r#"
        GtjSchemaNumber(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "number",
        )
        "#);
    }

    #[test]
    fn test_convert_string() {
        let string = GtjAny::GtjString(GtjString {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjStringTypeString,
        });
        assert_ron_snapshot!(string.to_schema(), @r#"
        GtjSchemaString(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "string",
        )
        "#);
    }

    #[test]
    fn test_convert_object() {
        let object = GtjAny::GtjObject(GtjObject {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjObjectTypeObject,
            properties: vec![GtjProperty {
                r#type: GtjPropertyTypeProperty,
                name: "foo".into(),
                doc: None,
                descriptor: GtjAny::GtjBoolean(GtjBoolean {
                    name: None,
                    doc: None,
                    r#type: GtjBooleanTypeBoolean,
                }),
                required: true,
            }],
        });
        assert_ron_snapshot!(object.to_schema(), @r#"
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

    #[test]
    fn test_convert_array() {
        let array = GtjAny::GtjArray(Box::new(GtjArray {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjNumber(GtjNumber {
                name: None,
                doc: None,
                r#type: GtjNumberTypeNumber,
            }),
        }));
        assert_ron_snapshot!(array.to_schema(), @r#"
        GtjSchemaArray(
          title: Some("hello"),
          description: Some("Hello, world!"),
          type: "array",
          items: GtjSchemaNumber(
            type: "number",
          ),
        )
        "#);
    }

    #[test]
    fn test_convert_union() {
        let union = GtjAny::GtjUnion(GtjUnion {
            r#type: GtjUnionTypeUnion,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            descriptors: vec![
                GtjAny::GtjNumber(GtjNumber {
                    name: None,
                    doc: None,
                    r#type: GtjNumberTypeNumber,
                }),
                GtjAny::GtjBoolean(GtjBoolean {
                    name: None,
                    doc: None,
                    r#type: GtjBooleanTypeBoolean,
                }),
            ],
        });
        assert_ron_snapshot!(union.to_schema(), @r#"
        GtjSchemaUnion(
          title: Some("hello"),
          description: Some("Hello, world!"),
          anyOf: [
            GtjSchemaNumber(
              type: "number",
            ),
            GtjSchemaBoolean(
              type: "boolean",
            ),
          ],
        )
        "#);
    }

    #[test]
    fn test_convert_literal() {
        let literal = GtjAny::GtjLiteral(GtjLiteral {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjLiteralTypeLiteral,
            value: GtjLiteralValue::Boolean(true),
        });
        assert_ron_snapshot!(literal.to_schema(), @r#"
        GtjSchemaLiteral(
          title: Some("hello"),
          description: Some("Hello, world!"),
          const: true,
        )
        "#);
    }

    #[test]
    fn test_convert_literal_null() {
        let literal = GtjAny::GtjLiteral(GtjLiteral {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjLiteralTypeLiteral,
            value: GtjLiteralValue::Null,
        });
        assert_ron_snapshot!(literal.to_schema(), @r#"
        GtjSchemaLiteral(
          title: Some("hello"),
          description: Some("Hello, world!"),
          const: (),
        )
        "#);
    }

    #[test]
    fn test_convert_tuple() {
        let tuple = GtjAny::GtjTuple(GtjTuple {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjTupleTypeTuple,
            descriptors: vec![GtjAny::GtjBoolean(GtjBoolean {
                name: None,
                doc: None,
                r#type: GtjBooleanTypeBoolean,
            })],
        });
        assert_ron_snapshot!(tuple.to_schema(), @r#"
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
