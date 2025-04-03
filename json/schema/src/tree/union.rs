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

impl GtjSchemaConvert<GtjSchemaUnion> for GtjUnion {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaUnion {
        GtjSchemaUnion {
            title: self.name.clone(),
            description: self.doc.clone(),
            any_of: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(_context))
                .collect(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjUnion {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaAny {
        GtjSchemaAny::Union(self.convert(_context))
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
            union.convert(&mut GtjSchemaConvertContext {}),
        );
    }
}
