use crate::*;
use genotype_json_types::*;

impl GtjSchemaConvert<GtjSchemaTuple> for GtjTuple {
    fn to_schema(&self) -> GtjSchemaTuple {
        GtjSchemaTuple {
            r#type: GtjSchemaTupleTypeArray,
            title: self.name.clone(),
            description: self.doc.clone(),
            prefix_items: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.to_schema())
                .collect(),
        }
    }
}

impl GtjSchemaConvert<GtjSchemaAny> for GtjTuple {
    fn to_schema(&self) -> GtjSchemaAny {
        GtjSchemaAny::GtjSchemaTuple(self.to_schema())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let tuple = GtjTuple {
            name: Some("hello".into()),
            doc: Some("Hello, world!".into()),
            r#type: GtjTupleTypeTuple,
            descriptors: vec![GtjAny::GtjBoolean(GtjBoolean {
                name: None,
                doc: None,
                r#type: GtjBooleanTypeBoolean,
            })],
        };
        assert_eq!(
            GtjSchemaTuple {
                r#type: GtjSchemaTupleTypeArray,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
                prefix_items: vec![GtjSchemaAny::GtjSchemaBoolean(GtjSchemaBoolean {
                    r#type: GtjSchemaBooleanTypeBoolean,
                    title: None,
                    description: None,
                })],
            },
            tuple.to_schema(),
        );
    }
}
