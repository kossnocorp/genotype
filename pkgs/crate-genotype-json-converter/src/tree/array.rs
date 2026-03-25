use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GtArray> for GtjArray {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GtArray {
        let name = context.claim_name(self.name.clone(), "Array");

        let descriptor =
            context.enter_name_context(GtNamingContextName::Identifier(name.clone()), |context| {
                context.enter_name_context(
                    GtNamingContextName::Transitive("Element".into()),
                    |context| self.descriptor.to_tree_with_context(context),
                )
            });

        GtArray {
            span: GtSpan::default(),
            doc: None,
            attributes: vec![],
            descriptor,
        }
    }
}

impl GtjTreeConvert<GtDescriptor> for GtjArray {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GtDescriptor {
        GtDescriptor::Array(Box::new(self.to_tree_with_context(context)))
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

        let descriptor_tree: GtDescriptor = array.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @"
        Array(GtArray(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(0, 0),
            kind: Number,
            doc: None,
            attributes: [],
          )),
        ))
        ");

        let array_tree: GtArray = array.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(array_tree, @"
        GtArray(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(0, 0),
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

        let tree: GtDescriptor = array.to_tree_with_context(&mut context);
        assert_ron_snapshot!(tree, @r#"
        Array(GtArray(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          descriptor: Object(GtObject(
            span: GtSpan(0, 0),
            doc: None,
            attributes: [],
            name: Named(GtIdentifier(GtSpan(0, 0), "RootElement")),
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

        let tree: GtArray = array.to_tree_with_context(&mut context);
        assert_ron_snapshot!(tree, @r#"
        GtArray(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          descriptor: Array(GtArray(
            span: GtSpan(0, 0),
            doc: None,
            attributes: [],
            descriptor: Object(GtObject(
              span: GtSpan(0, 0),
              doc: None,
              attributes: [],
              name: Named(GtIdentifier(GtSpan(0, 0), "RootElementElement")),
              extensions: [],
              properties: [
                GtProperty(
                  span: GtSpan(0, 0),
                  doc: None,
                  attributes: [],
                  name: GtKey(GtSpan(0, 0), "world"),
                  descriptor: Object(GtObject(
                    span: GtSpan(0, 0),
                    doc: None,
                    attributes: [],
                    name: Named(GtIdentifier(GtSpan(0, 0), "RootElementElementWorld")),
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

        let tree: GtArray = array.to_tree_with_context(&mut context);
        assert_ron_snapshot!(tree, @r#"
        GtArray(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          descriptor: Object(GtObject(
            span: GtSpan(0, 0),
            doc: None,
            attributes: [],
            name: Named(GtIdentifier(GtSpan(0, 0), "HelloElement")),
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

        let tree: GtArray = array.to_tree_with_context(&mut context);
        assert_ron_snapshot!(tree, @r#"
        GtArray(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          descriptor: Array(GtArray(
            span: GtSpan(0, 0),
            doc: None,
            attributes: [],
            descriptor: Object(GtObject(
              span: GtSpan(0, 0),
              doc: None,
              attributes: [],
              name: Named(GtIdentifier(GtSpan(0, 0), "Hey")),
              extensions: [],
              properties: [
                GtProperty(
                  span: GtSpan(0, 0),
                  doc: None,
                  attributes: [],
                  name: GtKey(GtSpan(0, 0), "world"),
                  descriptor: Object(GtObject(
                    span: GtSpan(0, 0),
                    doc: None,
                    attributes: [],
                    name: Named(GtIdentifier(GtSpan(0, 0), "HeyWorld")),
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
