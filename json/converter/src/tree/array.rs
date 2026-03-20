use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTArray> for GtjArray {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTArray {
        let name = context.claim_name(self.name.clone(), "Array");

        let descriptor =
            context.enter_name_context(GTNamingContextName::Identifier(name.clone()), |context| {
                context.enter_name_context(
                    GTNamingContextName::Transitive("Element".into()),
                    |context| self.descriptor.to_tree_with_context(context),
                )
            });

        GTArray {
            span: GTSpan::default(),
            descriptor,
        }
    }
}

impl GtjTreeConvert<GTDescriptor> for GtjArray {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTDescriptor {
        GTDescriptor::Array(Box::new(self.to_tree_with_context(context)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let array = GtjArray {
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjNumber(GtjNumber {
                name: None,
                doc: None,
                r#type: GtjNumberTypeNumber,
            }),
            name: None,
            doc: None,
        };

        let descriptor_tree: GTDescriptor = array.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @"
        Array(GTArray(
          span: GTSpan(0, 0),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(0, 0),
            kind: Number,
            doc: None,
            attributes: [],
          )),
        ))
        ");

        let array_tree: GTArray = array.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(array_tree, @"
        GTArray(
          span: GTSpan(0, 0),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(0, 0),
            kind: Number,
            doc: None,
            attributes: [],
          )),
        )
        ");
    }

    #[test]
    fn test_convert_naming_unnamed() {
        let mut context = GtjTreeConvertContext::new();

        let array = GtjArray {
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjObject(GtjObject {
                r#type: GtjObjectTypeObject,
                name: None,
                doc: None,
                properties: vec![],
            }),
            name: None,
            doc: None,
        };

        let tree: GTDescriptor = array.to_tree_with_context(&mut context);
        assert_ron_snapshot!(tree, @r#"
        Array(GTArray(
          span: GTSpan(0, 0),
          descriptor: Object(GTObject(
            span: GTSpan(0, 0),
            doc: None,
            attributes: [],
            name: Named(GTIdentifier(GTSpan(0, 0), "RootElement")),
            extensions: [],
            properties: [],
          )),
        ))
        "#);
    }

    #[test]
    fn test_convert_naming_unnamed_nested() {
        let mut context = GtjTreeConvertContext::new();

        let array = GtjArray {
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjArray(Box::new(GtjArray {
                r#type: GtjArrayTypeArray,
                descriptor: GtjAny::GtjObject(GtjObject {
                    r#type: GtjObjectTypeObject,
                    name: None,
                    doc: None,
                    properties: vec![GtjProperty {
                        r#type: GtjPropertyTypeProperty,
                        name: "world".into(),
                        doc: None,
                        descriptor: GtjAny::GtjObject(GtjObject {
                            name: None,
                            doc: None,
                            r#type: GtjObjectTypeObject,
                            properties: vec![],
                        }),
                        required: false,
                    }],
                }),
                name: None,
                doc: None,
            })),
            name: None,
            doc: None,
        };

        let tree: GTArray = array.to_tree_with_context(&mut context);
        assert_ron_snapshot!(tree, @r#"
        GTArray(
          span: GTSpan(0, 0),
          descriptor: Array(GTArray(
            span: GTSpan(0, 0),
            descriptor: Object(GTObject(
              span: GTSpan(0, 0),
              doc: None,
              attributes: [],
              name: Named(GTIdentifier(GTSpan(0, 0), "RootElementElement")),
              extensions: [],
              properties: [
                GTProperty(
                  span: GTSpan(0, 0),
                  doc: None,
                  attributes: [],
                  name: GTKey(GTSpan(0, 0), "world"),
                  descriptor: Object(GTObject(
                    span: GTSpan(0, 0),
                    doc: None,
                    attributes: [],
                    name: Named(GTIdentifier(GTSpan(0, 0), "RootElementElementWorld")),
                    extensions: [],
                    properties: [],
                  )),
                  required: false,
                ),
              ],
            )),
          )),
        )
        "#);
    }

    #[test]
    fn test_convert_naming_named() {
        let mut context = GtjTreeConvertContext::new();

        let array = GtjArray {
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjObject(GtjObject {
                r#type: GtjObjectTypeObject,
                name: None,
                doc: None,
                properties: vec![],
            }),
            name: Some("Hello".into()),
            doc: None,
        };

        let tree: GTArray = array.to_tree_with_context(&mut context);
        assert_ron_snapshot!(tree, @r#"
        GTArray(
          span: GTSpan(0, 0),
          descriptor: Object(GTObject(
            span: GTSpan(0, 0),
            doc: None,
            attributes: [],
            name: Named(GTIdentifier(GTSpan(0, 0), "HelloElement")),
            extensions: [],
            properties: [],
          )),
        )
        "#);
    }

    #[test]
    fn test_convert_naming_named_nested() {
        let mut context = GtjTreeConvertContext::new();

        let array = GtjArray {
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjArray(Box::new(GtjArray {
                r#type: GtjArrayTypeArray,
                descriptor: GtjAny::GtjObject(GtjObject {
                    r#type: GtjObjectTypeObject,
                    name: Some("Hey".into()),
                    doc: None,
                    properties: vec![GtjProperty {
                        r#type: GtjPropertyTypeProperty,
                        name: "world".into(),
                        doc: None,
                        descriptor: GtjAny::GtjObject(GtjObject {
                            name: None,
                            doc: None,
                            r#type: GtjObjectTypeObject,
                            properties: vec![],
                        }),
                        required: false,
                    }],
                }),
                name: Some("Hi".into()),
                doc: None,
            })),
            name: Some("Hello".into()),
            doc: None,
        };

        let tree: GTArray = array.to_tree_with_context(&mut context);
        assert_ron_snapshot!(tree, @r#"
        GTArray(
          span: GTSpan(0, 0),
          descriptor: Array(GTArray(
            span: GTSpan(0, 0),
            descriptor: Object(GTObject(
              span: GTSpan(0, 0),
              doc: None,
              attributes: [],
              name: Named(GTIdentifier(GTSpan(0, 0), "Hey")),
              extensions: [],
              properties: [
                GTProperty(
                  span: GTSpan(0, 0),
                  doc: None,
                  attributes: [],
                  name: GTKey(GTSpan(0, 0), "world"),
                  descriptor: Object(GTObject(
                    span: GTSpan(0, 0),
                    doc: None,
                    attributes: [],
                    name: Named(GTIdentifier(GTSpan(0, 0), "HeyWorld")),
                    extensions: [],
                    properties: [],
                  )),
                  required: false,
                ),
              ],
            )),
          )),
        )
        "#);
    }
}
