use crate::*;
use genotype_json_tree::*;
use literals::literal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaString {
    pub r#type: GtjSchemaStringType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[literal("string")]
pub struct GtjSchemaStringType;

impl GtjSchemaConvert<GtjSchemaString> for GtjString {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaString {
        GtjSchemaString {
            r#type: GtjSchemaStringType,
            title: self.name.clone(),
            description: self.doc.clone(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjString {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaAny {
        GtjSchemaAny::String(self.convert(_context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let string = GtjString {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjStringKindString,
        };
        assert_eq!(
            GtjSchemaString {
                r#type: GtjSchemaStringType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            },
            string.convert(&mut GtjSchemaConvertContext {}),
        );
    }
}
