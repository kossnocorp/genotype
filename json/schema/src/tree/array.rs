use crate::*;
use genotype_json_tree::*;
use literals::literal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaArray {
    pub r#type: GtjSchemaArrayType,
    pub items: GtjSchemaAny,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[literal("array")]
pub struct GtjSchemaArrayType;

impl From<GtjArray> for GtjSchemaArray {
    fn from(array: GtjArray) -> GtjSchemaArray {
        GtjSchemaArray {
            r#type: GtjSchemaArrayType,
            title: array.name.clone(),
            description: array.doc.clone(),
            items: array.descriptor.into(),
        }
    }
}

impl From<Box<GtjArray>> for GtjSchemaAny {
    fn from(array: Box<GtjArray>) -> GtjSchemaAny {
        GtjSchemaAny::Array(Box::new((*array).into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let array = GtjArray {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjArrayKindArray,
            descriptor: GtjAny::GtjBoolean(GtjBoolean {
                name: None,
                doc: None,
                kind: GtjBooleanKindBoolean,
            }),
        };
        assert_eq!(
            GtjSchemaArray {
                r#type: GtjSchemaArrayType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                items: GtjSchemaAny::Boolean(GtjSchemaBoolean {
                    r#type: GtjSchemaBooleanType,
                    title: None,
                    description: None,
                }),
            },
            array.into(),
        );
    }
}
