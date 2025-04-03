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

impl GtjSchemaConvert<GtjSchemaNull> for GtjNull {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaNull {
        GtjSchemaNull {
            r#type: GtjSchemaNullType,
            title: self.name.clone(),
            description: self.doc.clone(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjNull {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaAny {
        GtjSchemaAny::Null(self.convert(_context))
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
            null.convert(&mut GtjSchemaConvertContext {}),
        );
    }
}
