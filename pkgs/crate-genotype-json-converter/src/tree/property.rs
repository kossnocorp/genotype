use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GtProperty> for GtjProperty {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GtProperty {
        let descriptor = context.enter_name_context(
            GtNamingContextName::Transitive(self.name.clone()),
            |context| self.descriptor.to_tree_with_context(context),
        );

        GtProperty {
            span: GtSpan::default(),
            descriptor,
            attributes: Default::default(),
            required: self.required,
            name: GtKey(Default::default(), self.name.clone().into()),
            doc: self
                .doc
                .clone()
                .and_then(|content| Some(GtDoc(Default::default(), content))),
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

        let property_tree: GtProperty = property.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(property_tree, @r#"
        GtProperty(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          name: GtKey(GtSpan(0, 0), "hello"),
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(0, 0),
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

        let property_tree: GtProperty = property.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(property_tree, @r#"
        GtProperty(
          span: GtSpan(0, 0),
          doc: Some(GtDoc(GtSpan(0, 0), "Hello, world!")),
          attributes: [],
          name: GtKey(GtSpan(0, 0), "world"),
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(0, 0),
            kind: Number,
            doc: None,
            attributes: [],
          )),
          required: true,
        )
        "#);
    }
}
