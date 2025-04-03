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

impl From<GtjObject> for GtjSchemaObject {
    fn from(object: GtjObject) -> GtjSchemaObject {
        let mut required = vec![];
        let properties = object
            .properties
            .iter()
            .map(|property| {
                let name = property.name.clone();
                let schema = property.descriptor.clone().into();
                if property.required {
                    required.push(name.clone());
                }
                (name, schema)
            })
            .collect();
        GtjSchemaObject {
            r#type: GtjSchemaObjectType,
            title: object.name.clone(),
            description: object.doc.clone(),
            properties,
            required: Some(required),
            additional_properties: false,
        }
    }
}

impl From<GtjObject> for GtjSchemaAny {
    fn from(object: GtjObject) -> GtjSchemaAny {
        GtjSchemaAny::Object(object.into())
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
            object.into(),
        );
    }
}
