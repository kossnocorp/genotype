use crate::*;
use genotype_json_tree::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtjSchemaAny {
    Null(GtjSchemaNull),
    Boolean(GtjSchemaBoolean),
    Number(GtjSchemaNumber),
    String(GtjSchemaString),
    Object(GtjSchemaObject),
    Array(Box<GtjSchemaArray>),
    Union(GtjSchemaUnion),
    Literal(GtjSchemaConst),
    Tuple(GtjSchemaTuple),
}

impl From<GtjAny> for GtjSchemaAny {
    fn from(any: GtjAny) -> GtjSchemaAny {
        match any {
            GtjAny::GtjNull(null) => null.into(),
            GtjAny::GtjBoolean(boolean) => boolean.into(),
            GtjAny::GtjNumber(number) => number.into(),
            GtjAny::GtjString(string) => string.into(),
            GtjAny::GtjObject(object) => object.into(),
            GtjAny::GtjArray(array) => array.into(),
            GtjAny::GtjUnion(union) => union.into(),
            GtjAny::GtjLiteral(literal) => literal.into(),
            GtjAny::GtjTuple(tuple) => tuple.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_null() {
        let null = GtjAny::GtjNull(GtjNull {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjNullKindNull,
        });
        assert_eq!(
            GtjSchemaAny::Null(GtjSchemaNull {
                r#type: GtjSchemaNullType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            }),
            null.into(),
        );
    }

    #[test]
    fn test_convert_boolean() {
        let boolean = GtjAny::GtjBoolean(GtjBoolean {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjBooleanKindBoolean,
        });
        assert_eq!(
            GtjSchemaAny::Boolean(GtjSchemaBoolean {
                r#type: GtjSchemaBooleanType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            }),
            boolean.into(),
        );
    }

    #[test]
    fn test_convert_number() {
        let number = GtjAny::GtjNumber(GtjNumber {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjNumberKindNumber,
        });
        assert_eq!(
            GtjSchemaAny::Number(GtjSchemaNumber {
                r#type: GtjSchemaNumberType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            }),
            number.into(),
        );
    }

    #[test]
    fn test_convert_string() {
        let string = GtjAny::GtjString(GtjString {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjStringKindString,
        });
        assert_eq!(
            GtjSchemaAny::String(GtjSchemaString {
                r#type: GtjSchemaStringType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            }),
            string.into(),
        );
    }

    #[test]
    fn test_convert_object() {
        let object = GtjAny::GtjObject(GtjObject {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjObjectKindObject,
            properties: vec![GtjProperty {
                kind: GtjPropertyKindProperty,
                name: "foo".into(),
                doc: None,
                descriptor: GtjAny::GtjBoolean(GtjBoolean {
                    name: None,
                    doc: None,
                    kind: GtjBooleanKindBoolean,
                }),
                required: true,
            }],
        });
        assert_eq!(
            GtjSchemaAny::Object(GtjSchemaObject {
                r#type: GtjSchemaObjectType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                properties: HashMap::from_iter(vec![(
                    "foo".into(),
                    GtjSchemaAny::Boolean(GtjSchemaBoolean {
                        r#type: GtjSchemaBooleanType,
                        title: None,
                        description: None,
                    }),
                )]),
                required: Some(vec!["foo".into()]),
                additional_properties: false,
            }),
            object.into(),
        );
    }

    #[test]
    fn test_convert_array() {
        let array = GtjAny::GtjArray(Box::new(GtjArray {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjArrayKindArray,
            descriptor: GtjAny::GtjNull(GtjNull {
                name: None,
                doc: None,
                kind: GtjNullKindNull,
            }),
        }));
        assert_eq!(
            GtjSchemaAny::Array(Box::new(GtjSchemaArray {
                r#type: GtjSchemaArrayType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                items: GtjSchemaAny::Null(GtjSchemaNull {
                    r#type: GtjSchemaNullType,
                    title: None,
                    description: None,
                }),
            })),
            array.into(),
        );
    }

    #[test]
    fn test_convert_union() {
        let union = GtjAny::GtjUnion(GtjUnion {
            kind: GtjUnionKindUnion,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            descriptors: vec![
                GtjAny::GtjNull(GtjNull {
                    name: None,
                    doc: None,
                    kind: GtjNullKindNull,
                }),
                GtjAny::GtjBoolean(GtjBoolean {
                    name: None,
                    doc: None,
                    kind: GtjBooleanKindBoolean,
                }),
            ],
        });
        assert_eq!(
            GtjSchemaAny::Union(GtjSchemaUnion {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                any_of: vec![
                    GtjSchemaAny::Null(GtjSchemaNull {
                        r#type: GtjSchemaNullType,
                        title: None,
                        description: None,
                    }),
                    GtjSchemaAny::Boolean(GtjSchemaBoolean {
                        r#type: GtjSchemaBooleanType,
                        title: None,
                        description: None,
                    }),
                ],
            }),
            union.into(),
        );
    }

    #[test]
    fn test_convert_literal() {
        let literal = GtjAny::GtjLiteral(GtjLiteral {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjLiteralKindLiteral,
            value: GtjLiteralValue::Boolean(true),
        });
        assert_eq!(
            GtjSchemaAny::Literal(GtjSchemaConst {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                r#const: GtjSchemaConstValue::Boolean(true),
            }),
            literal.into(),
        );
    }

    #[test]
    fn test_convert_tuple() {
        let tuple = GtjAny::GtjTuple(GtjTuple {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjTupleKindTuple,
            descriptors: vec![GtjAny::GtjBoolean(GtjBoolean {
                name: None,
                doc: None,
                kind: GtjBooleanKindBoolean,
            })],
        });
        assert_eq!(
            GtjSchemaAny::Tuple(GtjSchemaTuple {
                r#type: GtjSchemaArrayType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                prefix_items: vec![GtjSchemaAny::Boolean(GtjSchemaBoolean {
                    r#type: GtjSchemaBooleanType,
                    title: None,
                    description: None,
                })],
            }),
            tuple.into(),
        );
    }
}
