use crate::*;
use genotype_json_tree::*;
use literals::literal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaNumber {
    pub r#type: GtjSchemaNumberType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[literal("number")]
pub struct GtjSchemaNumberType;

impl From<GtjNumber> for GtjSchemaNumber {
    fn from(number: GtjNumber) -> GtjSchemaNumber {
        GtjSchemaNumber {
            r#type: GtjSchemaNumberType,
            title: number.name.clone(),
            description: number.doc.clone(),
        }
    }
}

impl From<GtjNumber> for GtjSchemaAny {
    fn from(number: GtjNumber) -> GtjSchemaAny {
        GtjSchemaAny::Number(number.into())
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
            kind: GtjNumberKindNumber,
        };
        assert_eq!(
            GtjSchemaNumber {
                r#type: GtjSchemaNumberType,
                title: Some("hello".into()),
                description: Some("Hello, world!".into()),
            },
            number.into(),
        );
    }
}
