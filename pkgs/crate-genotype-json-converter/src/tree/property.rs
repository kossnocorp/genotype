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
            name: GTKey(Default::default(), self.name.clone().into()),
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
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let property = GtjProperty {
            r#type: GtjPropertyTypeProperty,
            name: "hello".into(),
            doc: None,
            descriptor: GtjAny::GtjNumber(GtjNumber {
                name: None,
                doc: None,
                r#type: GtjNumberTypeNumber,
            }),
            required: false,
        };

        let property_tree: GTProperty = property.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(property_tree, @r#"
        GTProperty(
          span: GTSpan(0, 0),
          doc: None,
          attributes: [],
          name: GTKey(GTSpan(0, 0), "hello"),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(0, 0),
            kind: Number,
            doc: None,
            attributes: [],
          )),
          required: false,
        )
        "#);

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

        let property_tree: GTProperty = property.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(property_tree, @r#"
        GTProperty(
          span: GTSpan(0, 0),
          doc: Some(GTDoc(GTSpan(0, 0), "Hello, world!")),
          attributes: [],
          name: GTKey(GTSpan(0, 0), "world"),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(0, 0),
            kind: Number,
            doc: None,
            attributes: [],
          )),
          required: true,
        )
        "#);
    }
}
