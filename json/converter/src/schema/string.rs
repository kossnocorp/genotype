use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaString> for GtjString {
    fn to_schema(&self) -> GtjSchemaString {
        GtjSchemaString {
            r#type: GtjSchemaStringTypeString,
            title: self.name.clone(),
            description: self.doc.clone(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjString {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaString(self.to_schema())
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
            r#type: GtjStringTypeString,
        };
        assert_eq!(
            GtjSchemaString {
                r#type: GtjSchemaStringTypeString,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            },
            string.to_schema(),
        );
    }
}
