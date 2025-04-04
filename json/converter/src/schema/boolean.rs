use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaBoolean> for GtjBoolean {
    fn to_schema(&self) -> GtjSchemaBoolean {
        GtjSchemaBoolean {
            r#type: GtjSchemaBooleanTypeBoolean,
            title: self.name.clone(),
            description: self.doc.clone(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjBoolean {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaBoolean(self.to_schema())
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
            r#type: GtjBooleanTypeBoolean,
        };
        assert_eq!(
            GtjSchemaBoolean {
                r#type: GtjSchemaBooleanTypeBoolean,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            },
            boolean.to_schema(),
        );
    }
}
