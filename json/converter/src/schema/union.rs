use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaUnion> for GtjUnion {
    fn to_schema(&self) -> GtjSchemaUnion {
        GtjSchemaUnion {
            title: self.name.clone(),
            description: self.doc.clone(),
            any_of: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.to_schema())
                .collect(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjUnion {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaUnion(self.to_schema())
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
            r#type: GtjUnionTypeUnion,
            descriptors: vec![GtjAny::GtjBoolean(GtjBoolean {
                name: None,
                doc: None,
                r#type: GtjBooleanTypeBoolean,
            })],
        };
        assert_eq!(
            GtjSchemaUnion {
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                any_of: vec![GtjSchemaAny::GtjSchemaBoolean(GtjSchemaBoolean {
                    r#type: GtjSchemaBooleanTypeBoolean,
                    title: None,
                    description: None,
                })],
            },
            union.to_schema(),
        );
    }
}
