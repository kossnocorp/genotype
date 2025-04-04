use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaNull> for GtjNull {
    fn to_schema(&self) -> GtjSchemaNull {
        GtjSchemaNull {
            r#type: GtjSchemaNullTypeNull,
            title: self.name.clone(),
            description: self.doc.clone(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjNull {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaNull(self.to_schema())
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
            r#type: GtjNullTypeNull,
        };
        assert_eq!(
            GtjSchemaNull {
                r#type: GtjSchemaNullTypeNull,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            },
            null.to_schema(),
        );
    }
}
