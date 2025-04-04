use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaObject> for GtjObject {
    fn to_schema(&self) -> GtjSchemaObject {
        let mut required = vec![];
        let properties = self
            .properties
            .iter()
            .map(|property| {
                let name = property.name.clone();
                let schema = property.descriptor.to_schema();
                if property.required {
                    required.push(name.clone());
                }
                (name, schema)
            })
            .collect();
        GtjSchemaObject {
            r#type: GtjSchemaObjectTypeObject,
            title: self.name.clone(),
            description: self.doc.clone(),
            properties,
            required: Some(required),
            additional_properties: Some(false),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjObject {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaObject(self.to_schema())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let object = GtjObject {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjObjectTypeObject,
            properties: vec![{
                GtjProperty {
                    r#type: GtjPropertyTypeProperty,
                    name: "foo".into(),
                    doc: None,
                    descriptor: GtjAny::GtjBoolean(GtjBoolean {
                        name: None,
                        doc: None,
                        r#type: GtjBooleanTypeBoolean,
                    }),
                    required: true,
                }
            }],
        };
        assert_eq!(
            GtjSchemaObject {
                r#type: GtjSchemaObjectTypeObject,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                properties: BTreeMap::from_iter(vec![(
                    "foo".into(),
                    GtjSchemaAny::GtjSchemaBoolean(GtjSchemaBoolean {
                        r#type: GtjSchemaBooleanTypeBoolean,
                        title: None,
                        description: None,
                    })
                )]),
                required: Some(vec!["foo".into()]),
                additional_properties: Some(false)
            },
            object.to_schema(),
        );
    }
}
