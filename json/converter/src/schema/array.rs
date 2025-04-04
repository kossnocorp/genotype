use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaArray> for GtjArray {
    fn to_schema(&self) -> GtjSchemaArray {
        GtjSchemaArray {
            r#type: GtjSchemaArrayTypeArray,
            title: self.name.clone(),
            description: self.doc.clone(),
            items: self.descriptor.to_schema(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjArray {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaArray(Box::new(self.to_schema()))
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
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjBoolean(GtjBoolean {
                name: None,
                doc: None,
                r#type: GtjBooleanTypeBoolean,
            }),
        };
        assert_eq!(
            GtjSchemaArray {
                r#type: GtjSchemaArrayTypeArray,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                items: GtjSchemaAny::GtjSchemaBoolean(GtjSchemaBoolean {
                    r#type: GtjSchemaBooleanTypeBoolean,
                    title: None,
                    description: None,
                }),
            },
            array.to_schema(),
        );
    }
}
