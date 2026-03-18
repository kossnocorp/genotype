use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTTuple> for GtjTuple {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTTuple {
        let name = context.claim_name(self.name.clone(), "Tuple");

        let descriptors =
            context.enter_name_context(GTNamingContextName::Identifier(name.clone()), |context| {
                self.descriptors
                    .iter()
                    .map(|descriptor| {
                        context.enter_name_context(
                            GTNamingContextName::Transitive("Element".into()),
                            |context| descriptor.to_tree_with_context(context),
                        )
                    })
                    .collect()
            });

        GTTuple {
            span: GTSpan::default(),
            descriptors,
        }
    }
}

impl GtjTreeConvert<GTDescriptor> for GtjTuple {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTDescriptor {
        GTDescriptor::Tuple(self.to_tree_with_context(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            descriptors: vec![],
            name: None,
            doc: None,
        };

        let descriptor_tree: GTDescriptor = tuple.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @"
        Tuple(GTTuple(
          span: GTSpan(0, 0),
          descriptors: [],
        ))
        ");

        let tuple_tree: GTTuple = tuple.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(tuple_tree, @"
        GTTuple(
          span: GTSpan(0, 0),
          descriptors: [],
        )
        ");
    }

    #[test]
    fn test_convert_descriptors() {
        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            descriptors: vec![GtjAny::GtjNull(GtjNull {
                r#type: GtjNullTypeNull,
                name: None,
                doc: None,
            })],
            name: None,
            doc: None,
        };

        let tuple_tree: GTTuple = tuple.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(tuple_tree, @"
        GTTuple(
          span: GTSpan(0, 0),
          descriptors: [
            Primitive(Null(GTSpan(0, 0))),
          ],
        )
        ");
    }

    #[test]
    fn test_convert_naming_unnamed() {
        let mut context = GtjTreeConvertContext::new();

        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            descriptors: vec![GtjAny::GtjObject(GtjObject {
                r#type: GtjObjectTypeObject,
                name: None,
                doc: None,
                properties: vec![],
            })],
            name: None,
            doc: None,
        };

        let tuple_tree: GTTuple = tuple.to_tree_with_context(&mut context);
        assert_ron_snapshot!(tuple_tree, @r#"
        GTTuple(
          span: GTSpan(0, 0),
          descriptors: [
            Object(GTObject(
              span: GTSpan(0, 0),
              name: Named(GTIdentifier(GTSpan(0, 0), "RootElement")),
              extensions: [],
              properties: [],
            )),
          ],
        )
        "#);
    }

    #[test]
    fn test_convert_naming_unnamed_nested() {
        let mut context = GtjTreeConvertContext::new();

        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            descriptors: vec![GtjAny::GtjTuple(GtjTuple {
                r#type: GtjTupleTypeTuple,
                descriptors: vec![GtjAny::GtjObject(GtjObject {
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
                })],
                name: None,
                doc: None,
            })],
            name: None,
            doc: None,
        };

        let tuple_tree: GTTuple = tuple.to_tree_with_context(&mut context);
        assert_ron_snapshot!(tuple_tree, @r#"
        GTTuple(
          span: GTSpan(0, 0),
          descriptors: [
            Tuple(GTTuple(
              span: GTSpan(0, 0),
              descriptors: [
                Object(GTObject(
                  span: GTSpan(0, 0),
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
                        name: Named(GTIdentifier(GTSpan(0, 0), "RootElementElementWorld")),
                        extensions: [],
                        properties: [],
                      )),
                      required: false,
                    ),
                  ],
                )),
              ],
            )),
          ],
        )
        "#);
    }

    #[test]
    fn test_convert_naming_named() {
        let mut context = GtjTreeConvertContext::new();

        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            descriptors: vec![GtjAny::GtjObject(GtjObject {
                r#type: GtjObjectTypeObject,
                name: Some("Hello".into()),
                doc: None,
                properties: vec![],
            })],
            name: Some("World".into()),
            doc: None,
        };

        let tuple_tree: GTTuple = tuple.to_tree_with_context(&mut context);
        assert_ron_snapshot!(tuple_tree, @r#"
        GTTuple(
          span: GTSpan(0, 0),
          descriptors: [
            Object(GTObject(
              span: GTSpan(0, 0),
              name: Named(GTIdentifier(GTSpan(0, 0), "Hello")),
              extensions: [],
              properties: [],
            )),
          ],
        )
        "#);
    }

    #[test]
    fn test_convert_naming_named_nested() {
        let mut context = GtjTreeConvertContext::new();

        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            descriptors: vec![GtjAny::GtjTuple(GtjTuple {
                r#type: GtjTupleTypeTuple,
                descriptors: vec![GtjAny::GtjObject(GtjObject {
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
                })],
                name: Some("Hi".into()),
                doc: None,
            })],
            name: Some("Hello".into()),
            doc: None,
        };

        let tuple_tree: GTTuple = tuple.to_tree_with_context(&mut context);
        assert_ron_snapshot!(tuple_tree, @r#"
        GTTuple(
          span: GTSpan(0, 0),
          descriptors: [
            Tuple(GTTuple(
              span: GTSpan(0, 0),
              descriptors: [
                Object(GTObject(
                  span: GTSpan(0, 0),
                  name: Named(GTIdentifier(GTSpan(0, 0), "HiElement")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(0, 0),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(0, 0), "world"),
                      descriptor: Object(GTObject(
                        span: GTSpan(0, 0),
                        name: Named(GTIdentifier(GTSpan(0, 0), "HiElementWorld")),
                        extensions: [],
                        properties: [],
                      )),
                      required: false,
                    ),
                  ],
                )),
              ],
            )),
          ],
        )
        "#);
    }
}
