use crate::*;
use genotype_json_tree::*;
use literals::literal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaConst {
    pub r#const: GtjSchemaConstValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GtjSchemaConstValue {
    Null(()),
    Boolean(bool),
    Number(f64),
    String(String),
}

impl From<GtjLiteral> for GtjSchemaConst {
    fn from(literal: GtjLiteral) -> GtjSchemaConst {
        GtjSchemaConst {
            title: literal.name.clone(),
            description: literal.doc.clone(),
            r#const: match literal.value {
                GtjLiteralValue::Null(value) => GtjSchemaConstValue::Null(value),
                GtjLiteralValue::Boolean(value) => GtjSchemaConstValue::Boolean(value),
                GtjLiteralValue::Number(value) => GtjSchemaConstValue::Number(value),
                GtjLiteralValue::String(ref value) => GtjSchemaConstValue::String(value.clone()),
            },
        }
    }
}

impl From<GtjLiteral> for GtjSchemaAny {
    fn from(literal: GtjLiteral) -> GtjSchemaAny {
        GtjSchemaAny::Literal(literal.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_null() {
        let literal = GtjLiteral {
            kind: GtjLiteralKindLiteral,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            value: GtjLiteralValue::Null(()),
        };
        assert_eq!(
            GtjSchemaConst {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                r#const: GtjSchemaConstValue::Null(()),
            },
            literal.into(),
        );
    }

    #[test]
    fn test_convert_boolean() {
        let literal = GtjLiteral {
            kind: GtjLiteralKindLiteral,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            value: GtjLiteralValue::Boolean(true),
        };
        assert_eq!(
            GtjSchemaConst {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                r#const: GtjSchemaConstValue::Boolean(true),
            },
            literal.into(),
        );
    }

    #[test]
    fn test_convert_number() {
        let literal = GtjLiteral {
            kind: GtjLiteralKindLiteral,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            value: GtjLiteralValue::Number(42.0),
        };
        assert_eq!(
            GtjSchemaConst {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                r#const: GtjSchemaConstValue::Number(42.0),
            },
            literal.into(),
        );
    }

    #[test]
    fn test_convert_string() {
        let literal = GtjLiteral {
            kind: GtjLiteralKindLiteral,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            value: GtjLiteralValue::String("Hello".into()),
        };
        assert_eq!(
            GtjSchemaConst {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                r#const: GtjSchemaConstValue::String("Hello".into()),
            },
            literal.into(),
        );
    }
}
