use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaNumber> for GtjNumber {
    fn to_schema(&self) -> GtjSchemaNumber {
        GtjSchemaNumber {
            r#type: GtjSchemaNumberTypeNumber,
            title: self.name.clone(),
            description: self.doc.clone(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjNumber {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaNumber(self.to_schema())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let number = GtjNumber {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjNumberTypeNumber,
        };
        assert_eq!(
            GtjSchemaNumber {
                r#type: GtjSchemaNumberTypeNumber,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            },
            number.to_schema(),
        );
    }
}
