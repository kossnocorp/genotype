use std::collections::HashMap;

use crate::*;
use genotype_json_tree::*;
use literals::literal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaObject {
    pub r#type: GtjSchemaObjectType,
    pub properties: HashMap<String, GtjSchemaAny>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[literal("object")]
pub struct GtjSchemaObjectType;

impl GtjSchemaConvert<GtjSchemaObject> for GtjObject {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaObject {
        let mut required = vec![];
        let properties = self
            .properties
            .iter()
            .map(|property| {
                let name = property.name.clone();
                let schema = property.descriptor.convert(_context);
                if property.required {
                    required.push(name.clone());
                }
                (name, schema)
            })
            .collect();
        GtjSchemaObject {
            r#type: GtjSchemaObjectType,
            title: self.name.clone(),
            description: self.doc.clone(),
            properties,
            required: Some(required),
            additional_properties: false,
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjObject {
    fn convert(&self, _context: &mut GtjSchemaConvertContext) -> GtjSchemaAny {
        GtjSchemaAny::Object(self.convert(_context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let object = GtjObject {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            kind: GtjObjectKindObject,
            properties: vec![{
                GtjProperty {
                    kind: GtjPropertyKindProperty,
                    name: "foo".into(),
                    doc: None,
                    descriptor: GtjAny::GtjBoolean(GtjBoolean {
                        name: None,
                        doc: None,
                        kind: GtjBooleanKindBoolean,
                    }),
                    required: true,
                }
            }],
        };
        assert_eq!(
            GtjSchemaObject {
                r#type: GtjSchemaObjectType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                properties: HashMap::from_iter(vec![(
                    "foo".into(),
                    GtjSchemaAny::Boolean(GtjSchemaBoolean {
                        r#type: GtjSchemaBooleanType,
                        title: None,
                        description: None,
                    })
                )]),
                required: Some(vec!["foo".into()]),
                additional_properties: false
            },
            object.convert(&mut GtjSchemaConvertContext {}),
        );
    }
}
