use crate::*;
use genotype_json_tree::*;
use literals::literal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaBoolean {
    pub r#type: GtjSchemaBooleanType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[literal("boolean")]
pub struct GtjSchemaBooleanType;

impl GtjSchemaConvert<GtjSchemaBoolean> for GtjBoolean {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaBoolean {
        GtjSchemaBoolean {
            r#type: GtjSchemaBooleanType,
            title: self.name.clone(),
            description: self.doc.clone(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjBoolean {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaAny {
        GtjSchemaAny::Boolean(self.convert(_context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let boolean = GtjBoolean {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjBooleanKindBoolean,
        };
        assert_eq!(
            GtjSchemaBoolean {
                r#type: GtjSchemaBooleanType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            },
            boolean.convert(&mut GtjSchemaConvertContext {}),
        );
    }
}
