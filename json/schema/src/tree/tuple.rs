use crate::*;
use genotype_json_tree::*;
use literals::literal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaTuple {
    pub r#type: GtjSchemaArrayType,
    #[serde(rename = "prefixItems")]
    pub prefix_items: Vec<GtjSchemaAny>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl From<GtjTuple> for GtjSchemaTuple {
    fn from(tuple: GtjTuple) -> GtjSchemaTuple {
        GtjSchemaTuple {
            r#type: GtjSchemaArrayType,
            title: tuple.name.clone(),
            description: tuple.doc.clone(),
            prefix_items: tuple
                .descriptors
                .iter()
                .map(|descriptor| descriptor.clone().into())
                .collect(),
        }
    }
}

impl From<GtjTuple> for GtjSchemaAny {
    fn from(tuple: GtjTuple) -> GtjSchemaAny {
        GtjSchemaAny::Tuple(tuple.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let tuple = GtjTuple {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjTupleKindTuple,
            descriptors: vec![GtjAny::GtjBoolean(GtjBoolean {
                name: None,
                doc: None,
                kind: GtjBooleanKindBoolean,
            })],
        };
        assert_eq!(
            GtjSchemaTuple {
                r#type: GtjSchemaArrayType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                prefix_items: vec![GtjSchemaAny::Boolean(GtjSchemaBoolean {
                    r#type: GtjSchemaBooleanType,
                    title: None,
                    description: None,
                })],
            },
            tuple.into(),
        );
    }
}
