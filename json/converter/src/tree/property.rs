use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTProperty> for GtjProperty {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTProperty {
        let descriptor = context.enter_name_context(
            GTNamingContextName::Transitive(self.name.clone()),
            |context| self.descriptor.to_tree_with_context(context),
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let property = GtjProperty {
            r#type: GtjPropertyTypeProperty,
            name: "hello".into(),
            doc: None,
            descriptor: GtjAny::GtjNull(GtjNull {
                r#type: GtjNullTypeNull,
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
            property.to_tree_with_context(&mut Default::default()),
        );

        let property = GtjProperty {
            r#type: GtjPropertyTypeProperty,
            name: "world".into(),
            doc: Some("Hello, world!".into()),
            descriptor: GtjAny::GtjNumber(GtjNumber {
                r#type: GtjNumberTypeNumber,
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
            property.to_tree_with_context(&mut Default::default()),
        );
    }
}
