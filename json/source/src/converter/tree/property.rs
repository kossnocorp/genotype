use genotype_json_schema::json::*;
use genotype_parser::*;

use crate::{GtjConvert, GtjConvertContext};

impl GtjConvert<GTProperty> for GtjProperty {
    fn convert(&self, context: &mut GtjConvertContext) -> GTProperty {
        let descriptor = context.enter_name_context(
            GTNamingContextName::Transitive(self.name.clone()),
            |context| self.descriptor.convert(context),
        );

        GTProperty {
            span: GTSpan::default(),
            descriptor,
            attributes: Default::default(),
            required: self.required,
            name: GTKey(Default::default(), self.name.clone()),
            doc: self
                .doc
                .clone()
                .and_then(|content| Some(GTDoc(Default::default(), content))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_json_schema::json::{GtjAny, GtjNull, GtjNullKindNull, GtjPropertyKindProperty};
    use genotype_parser::GTProperty;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let property = GtjProperty {
            kind: GtjPropertyKindProperty,
            name: "hello".into(),
            doc: None,
            descriptor: GtjAny::GtjNull(GtjNull {
                kind: GtjNullKindNull,
                name: None,
                doc: None,
            }),
            required: false,
        };
        assert_eq!(
            GTProperty {
                span: Default::default(),
                descriptor: GTPrimitive::Null(Default::default()).into(),
                attributes: Default::default(),
                required: false,
                name: GTKey(Default::default(), "hello".into()),
                doc: None,
            },
            property.convert(&mut Default::default()),
        );

        let property = GtjProperty {
            kind: GtjPropertyKindProperty,
            name: "world".into(),
            doc: Some("Hello, world!".into()),
            descriptor: GtjAny::GtjNumber(GtjNumber {
                kind: GtjNumberKindNumber,
                name: None,
                doc: None,
            }),
            required: true,
        };
        assert_eq!(
            GTProperty {
                span: Default::default(),
                descriptor: GTPrimitive::Number(Default::default()).into(),
                attributes: Default::default(),
                required: true,
                name: GTKey(Default::default(), "world".into()),
                doc: Some(GTDoc(Default::default(), "Hello, world!".into())),
            },
            property.convert(&mut Default::default()),
        );
    }
}
