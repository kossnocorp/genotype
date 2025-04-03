use crate::*;
use genotype_json_tree::*;
use literals::literal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaArray {
    pub r#type: GtjSchemaArrayType,
    pub items: Box<GtjSchemaAny>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[literal("array")]
pub struct GtjSchemaArrayType;

impl GtjSchemaConvert<GtjSchemaArray> for GtjArray {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaArray {
        GtjSchemaArray {
            r#type: GtjSchemaArrayType,
            title: self.name.clone(),
            description: self.doc.clone(),
            items: Box::new(self.descriptor.convert(_context)),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjArray {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaAny {
        GtjSchemaAny::Array(self.convert(_context))
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
                items: Box::new(GtjSchemaAny::Boolean(GtjSchemaBoolean {
                    r#type: GtjSchemaBooleanType,
                    title: None,
                    description: None,
                })),
            },
            array.convert(&mut GtjSchemaConvertContext {}),
        );
    }
}
