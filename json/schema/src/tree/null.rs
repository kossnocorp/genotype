use crate::*;
use genotype_json_tree::*;
use literals::literal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaNull {
    pub r#type: GtjSchemaNullType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[literal("null")]
pub struct GtjSchemaNullType;

impl From<GtjNull> for GtjSchemaNull {
    fn from(null: GtjNull) -> GtjSchemaNull {
        GtjSchemaNull {
            r#type: GtjSchemaNullType,
            title: null.name.clone(),
            description: null.doc.clone(),
        }
    }
}

impl From<GtjNull> for GtjSchemaAny {
    fn from(null: GtjNull) -> GtjSchemaAny {
        GtjSchemaAny::Null(null.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let null = GtjNull {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjNullKindNull,
        };
        assert_eq!(
            GtjSchemaNull {
                r#type: GtjSchemaNullType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            },
            null.into(),
        );
    }
}
