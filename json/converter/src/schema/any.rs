use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaAny> for GtjAny {
    fn to_schema(&self) -> GtjSchemaAny {
        match self {
            GtjAny::GtjNull(null) => null.to_schema(),
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
    use std::collections::BTreeMap;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_null() {
        let null = GtjAny::GtjNull(GtjNull {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjNullTypeNull,
        });
        assert_eq!(
            GtjSchemaAny::GtjSchemaNull(GtjSchemaNull {
                r#type: GtjSchemaNullTypeNull,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            }),
            null.to_schema(),
        );
    }

    #[test]
    fn test_convert_boolean() {
        let boolean = GtjAny::GtjBoolean(GtjBoolean {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjBooleanTypeBoolean,
        });
        assert_eq!(
            GtjSchemaAny::GtjSchemaBoolean(GtjSchemaBoolean {
                r#type: GtjSchemaBooleanTypeBoolean,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            }),
            boolean.to_schema(),
        );
    }

    #[test]
    fn test_convert_number() {
        let number = GtjAny::GtjNumber(GtjNumber {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjNumberTypeNumber,
        });
        assert_eq!(
            GtjSchemaAny::GtjSchemaNumber(GtjSchemaNumber {
                r#type: GtjSchemaNumberTypeNumber,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            }),
            number.to_schema(),
        );
    }

    #[test]
    fn test_convert_string() {
        let string = GtjAny::GtjString(GtjString {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjStringTypeString,
        });
        assert_eq!(
            GtjSchemaAny::GtjSchemaString(GtjSchemaString {
                r#type: GtjSchemaStringTypeString,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            }),
            string.to_schema(),
        );
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
        assert_eq!(
            GtjSchemaAny::GtjSchemaObject(GtjSchemaObject {
                r#type: GtjSchemaObjectTypeObject,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                properties: BTreeMap::from_iter(vec![(
                    "foo".into(),
                    GtjSchemaAny::GtjSchemaBoolean(GtjSchemaBoolean {
                        r#type: GtjSchemaBooleanTypeBoolean,
                        title: None,
                        description: None,
                    }),
                )]),
                required: Some(vec!["foo".into()]),
                additional_properties: Some(false),
            }),
            object.to_schema(),
        );
    }

    #[test]
    fn test_convert_array() {
        let array = GtjAny::GtjArray(Box::new(GtjArray {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjNull(GtjNull {
                name: None,
                doc: None,
                r#type: GtjNullTypeNull,
            }),
        }));
        assert_eq!(
            GtjSchemaAny::GtjSchemaArray(Box::new(GtjSchemaArray {
                r#type: GtjSchemaArrayTypeArray,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                items: GtjSchemaAny::GtjSchemaNull(GtjSchemaNull {
                    r#type: GtjSchemaNullTypeNull,
                    title: None,
                    description: None,
                }),
            })),
            array.to_schema(),
        );
    }

    #[test]
    fn test_convert_union() {
        let union = GtjAny::GtjUnion(GtjUnion {
            r#type: GtjUnionTypeUnion,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            descriptors: vec![
                GtjAny::GtjNull(GtjNull {
                    name: None,
                    doc: None,
                    r#type: GtjNullTypeNull,
                }),
                GtjAny::GtjBoolean(GtjBoolean {
                    name: None,
                    doc: None,
                    r#type: GtjBooleanTypeBoolean,
                }),
            ],
        });
        assert_eq!(
            GtjSchemaAny::GtjSchemaUnion(GtjSchemaUnion {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                any_of: vec![
                    GtjSchemaAny::GtjSchemaNull(GtjSchemaNull {
                        r#type: GtjSchemaNullTypeNull,
                        title: None,
                        description: None,
                    }),
                    GtjSchemaAny::GtjSchemaBoolean(GtjSchemaBoolean {
                        r#type: GtjSchemaBooleanTypeBoolean,
                        title: None,
                        description: None,
                    }),
                ],
            }),
            union.to_schema(),
        );
    }

    #[test]
    fn test_convert_literal() {
        let literal = GtjAny::GtjLiteral(GtjLiteral {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjLiteralTypeLiteral,
            value: GtjLiteralValue::Boolean(true),
        });
        assert_eq!(
            GtjSchemaAny::GtjSchemaLiteral(GtjSchemaLiteral {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                r#const: GtjSchemaLiteralConst::Boolean(true),
            }),
            literal.to_schema(),
        );
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
        assert_eq!(
            GtjSchemaAny::GtjSchemaTuple(GtjSchemaTuple {
                r#type: GtjSchemaTupleTypeArray,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                prefix_items: vec![GtjSchemaAny::GtjSchemaBoolean(GtjSchemaBoolean {
                    r#type: GtjSchemaBooleanTypeBoolean,
                    title: None,
                    description: None,
                })],
            }),
            tuple.to_schema(),
        );
    }
}
