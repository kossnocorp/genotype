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

impl GtjSchemaConvert<GtjSchemaTuple> for GtjTuple {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaTuple {
        GtjSchemaTuple {
            r#type: GtjSchemaArrayType,
            title: self.name.clone(),
            description: self.doc.clone(),
            prefix_items: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(_context))
                .collect(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjTuple {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaAny {
        GtjSchemaAny::Tuple(self.convert(_context))
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
            tuple.convert(&mut GtjSchemaConvertContext {}),
        );
    }
}
