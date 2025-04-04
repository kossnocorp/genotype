use crate::*;
use genotype_json_types::*;
use literals::literal;
use serde::{Deserialize, Serialize};

impl GtjSchemaConvert<GtjSchemaLiteral> for GtjLiteral {
    fn to_schema(&self) -> GtjSchemaLiteral {
        GtjSchemaLiteral {
            title: self.name.clone(),
            description: self.doc.clone(),
            r#const: match self.value {
                GtjLiteralValue::Null(value) => GtjSchemaLiteralConst::Null(value),
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_null() {
        let literal = GtjLiteral {
            r#type: GtjLiteralTypeLiteral,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            value: GtjLiteralValue::Null(()),
        };
        assert_eq!(
            GtjSchemaLiteral {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                r#const: GtjSchemaLiteralConst::Null(()),
            },
            literal.to_schema(),
        );
    }

    #[test]
    fn test_convert_boolean() {
        let literal = GtjLiteral {
            r#type: GtjLiteralTypeLiteral,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            value: GtjLiteralValue::Boolean(true),
        };
        assert_eq!(
            GtjSchemaLiteral {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                r#const: GtjSchemaLiteralConst::Boolean(true),
            },
            literal.to_schema(),
        );
    }

    #[test]
    fn test_convert_number() {
        let literal = GtjLiteral {
            r#type: GtjLiteralTypeLiteral,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            value: GtjLiteralValue::Number(42.0),
        };
        assert_eq!(
            GtjSchemaLiteral {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                r#const: GtjSchemaLiteralConst::Number(42.0),
            },
            literal.to_schema(),
        );
    }

    #[test]
    fn test_convert_string() {
        let literal = GtjLiteral {
            r#type: GtjLiteralTypeLiteral,
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            value: GtjLiteralValue::String("Hello".into()),
        };
        assert_eq!(
            GtjSchemaLiteral {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                r#const: GtjSchemaLiteralConst::String("Hello".into()),
            },
            literal.to_schema(),
        );
    }
}
