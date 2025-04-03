use crate::*;
use genotype_json_tree::*;
use literals::literal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaUnion {
    #[serde(rename = "anyOf")]
    pub any_of: Vec<GtjSchemaAny>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl From<GtjUnion> for GtjSchemaUnion {
    fn from(union: GtjUnion) -> GtjSchemaUnion {
        GtjSchemaUnion {
            title: union.name.clone(),
            description: union.doc.clone(),
            any_of: union
                .descriptors
                .iter()
                .map(|descriptor| descriptor.clone().into())
                .collect(),
        }
    }
}

impl From<GtjUnion> for GtjSchemaAny {
    fn from(union: GtjUnion) -> GtjSchemaAny {
        GtjSchemaAny::Union(union.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let union = GtjUnion {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjUnionKindUnion,
            descriptors: vec![GtjAny::GtjBoolean(GtjBoolean {
                name: None,
                doc: None,
                kind: GtjBooleanKindBoolean,
            })],
        };
        assert_eq!(
            GtjSchemaUnion {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                any_of: vec![GtjSchemaAny::Boolean(GtjSchemaBoolean {
                    r#type: GtjSchemaBooleanType,
                    title: None,
                    description: None,
                })],
            },
            union.into(),
        );
    }
}
