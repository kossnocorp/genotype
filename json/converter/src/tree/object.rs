use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTObject> for GtjObject {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTObject {
        let name = context.claim_name(self.name.clone(), "Object");

        let properties =
            context.enter_name_context(GTNamingContextName::Identifier(name.clone()), |context| {
                self.properties
                    .iter()
                    .map(|property| property.to_tree_with_context(context))
                    .collect()
            });

        GTObject {
            span: Default::default(),
            name: GTObjectName::Named(GTIdentifier(Default::default(), name)),
            properties,
            extensions: vec![],
        }
    }
}

impl GtjTreeConvert<GTDescriptor> for GtjObject {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTDescriptor {
        GTDescriptor::Object(self.to_tree_with_context(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            name: Some("Hello".into()),
            doc: None,
            properties: vec![],
        };

        let descriptor_tree: GTDescriptor = object.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @r#"
        Object(GTObject(
          span: GTSpan(0, 0),
          name: Named(GTIdentifier(GTSpan(0, 0), "Hello")),
          extensions: [],
          properties: [],
        ))
        "#);

        let object_tree: GTObject = object.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(object_tree, @r#"
        GTObject(
          span: GTSpan(0, 0),
          name: Named(GTIdentifier(GTSpan(0, 0), "Hello")),
          extensions: [],
          properties: [],
        )
        "#);
    }

    #[test]
    fn test_convert_unnamed() {
        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            name: None,
            doc: None,
            properties: vec![],
        };

        let object_tree: GTObject = object.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(object_tree, @r#"
        GTObject(
          span: GTSpan(0, 0),
          name: Named(GTIdentifier(GTSpan(0, 0), "Root")),
          extensions: [],
          properties: [],
        )
        "#);
    }

    #[test]
    fn test_convert_properties() {
        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            name: None,
            doc: None,
            properties: vec![GtjProperty {
                r#type: GtjPropertyTypeProperty,
                name: "null".into(),
                doc: None,
                descriptor: GtjAny::GtjNumber(GtjNumber {
                    name: None,
                    doc: None,
                    r#type: GtjNumberTypeNumber,
                }),
                required: true,
            }],
        };

        let object_tree: GTObject = object.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(object_tree, @r#"
        GTObject(
          span: GTSpan(0, 0),
          name: Named(GTIdentifier(GTSpan(0, 0), "Root")),
          extensions: [],
          properties: [
            GTProperty(
              span: GTSpan(0, 0),
              doc: None,
              attributes: [],
              name: GTKey(GTSpan(0, 0), "null"),
              descriptor: Primitive(Number(GTSpan(0, 0))),
              required: true,
            ),
          ],
        )
        "#);
    }

    #[test]
    fn test_convert_naming_unnamed() {
        let mut context = GtjTreeConvertContext::new();

        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            properties: vec![GtjProperty {
                r#type: GtjPropertyTypeProperty,
                name: "hello".into(),
                doc: None,
                descriptor: GtjAny::GtjObject(GtjObject {
                    r#type: GtjObjectTypeObject,
                    name: None,
                    doc: None,
                    properties: vec![],
                }),
                required: false,
            }],
            name: None,
            doc: None,
        };

        let object_tree: GTObject = object.to_tree_with_context(&mut context);
        assert_ron_snapshot!(object_tree, @r#"
        GTObject(
          span: GTSpan(0, 0),
          name: Named(GTIdentifier(GTSpan(0, 0), "Root")),
          extensions: [],
          properties: [
            GTProperty(
              span: GTSpan(0, 0),
              doc: None,
              attributes: [],
              name: GTKey(GTSpan(0, 0), "hello"),
              descriptor: Object(GTObject(
                span: GTSpan(0, 0),
                name: Named(GTIdentifier(GTSpan(0, 0), "RootHello")),
                extensions: [],
                properties: [],
              )),
              required: false,
            ),
          ],
        )
        "#);
    }

    #[test]
    fn test_convert_naming_unnamed_nested() {
        let mut context = GtjTreeConvertContext::new();

        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            properties: vec![GtjProperty {
                r#type: GtjPropertyTypeProperty,
                name: "hello".into(),
                doc: None,
                descriptor: GtjAny::GtjObject(GtjObject {
                    r#type: GtjObjectTypeObject,
                    name: None,
                    doc: None,
                    properties: vec![GtjProperty {
                        r#type: GtjPropertyTypeProperty,
                        name: "world".into(),
                        doc: None,
                        descriptor: GtjAny::GtjObject(GtjObject {
                            r#type: GtjObjectTypeObject,
                            name: None,
                            doc: None,
                            properties: vec![],
                        }),
                        required: false,
                    }],
                }),
                required: false,
            }],
            name: None,
            doc: None,
        };

        let object_tree: GTObject = object.to_tree_with_context(&mut context);
        assert_ron_snapshot!(object_tree, @r#"
        GTObject(
          span: GTSpan(0, 0),
          name: Named(GTIdentifier(GTSpan(0, 0), "Root")),
          extensions: [],
          properties: [
            GTProperty(
              span: GTSpan(0, 0),
              doc: None,
              attributes: [],
              name: GTKey(GTSpan(0, 0), "hello"),
              descriptor: Object(GTObject(
                span: GTSpan(0, 0),
                name: Named(GTIdentifier(GTSpan(0, 0), "RootHello")),
                extensions: [],
                properties: [
                  GTProperty(
                    span: GTSpan(0, 0),
                    doc: None,
                    attributes: [],
                    name: GTKey(GTSpan(0, 0), "world"),
                    descriptor: Object(GTObject(
                      span: GTSpan(0, 0),
                      name: Named(GTIdentifier(GTSpan(0, 0), "RootHelloWorld")),
                      extensions: [],
                      properties: [],
                    )),
                    required: false,
                  ),
                ],
              )),
              required: false,
            ),
          ],
        )
        "#);
    }

    #[test]
    fn test_convert_naming_named() {
        let mut context = GtjTreeConvertContext::new();

        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            properties: vec![GtjProperty {
                r#type: GtjPropertyTypeProperty,
                name: "world".into(),
                doc: None,
                descriptor: GtjAny::GtjObject(GtjObject {
                    r#type: GtjObjectTypeObject,
                    name: None,
                    doc: None,
                    properties: vec![],
                }),
                required: false,
            }],
            name: Some("Hello".into()),
            doc: None,
        };

        let object_tree: GTObject = object.to_tree_with_context(&mut context);
        assert_ron_snapshot!(object_tree, @r#"
        GTObject(
          span: GTSpan(0, 0),
          name: Named(GTIdentifier(GTSpan(0, 0), "Hello")),
          extensions: [],
          properties: [
            GTProperty(
              span: GTSpan(0, 0),
              doc: None,
              attributes: [],
              name: GTKey(GTSpan(0, 0), "world"),
              descriptor: Object(GTObject(
                span: GTSpan(0, 0),
                name: Named(GTIdentifier(GTSpan(0, 0), "HelloWorld")),
                extensions: [],
                properties: [],
              )),
              required: false,
            ),
          ],
        )
        "#);
    }

    #[test]
    fn test_convert_naming_named_nested() {
        let mut context = GtjTreeConvertContext::new();

        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            properties: vec![GtjProperty {
                r#type: GtjPropertyTypeProperty,
                name: "world".into(),
                doc: None,
                descriptor: GtjAny::GtjObject(GtjObject {
                    r#type: GtjObjectTypeObject,
                    name: Some("Hi".into()),
                    doc: None,
                    properties: vec![GtjProperty {
                        r#type: GtjPropertyTypeProperty,
                        name: "world".into(),
                        doc: None,
                        descriptor: GtjAny::GtjObject(GtjObject {
                            r#type: GtjObjectTypeObject,
                            name: None,
                            doc: None,
                            properties: vec![],
                        }),
                        required: false,
                    }],
                }),
                required: false,
            }],
            name: Some("Hello".into()),
            doc: None,
        };

        let object_tree: GTObject = object.to_tree_with_context(&mut context);
        assert_ron_snapshot!(object_tree, @r#"
        GTObject(
          span: GTSpan(0, 0),
          name: Named(GTIdentifier(GTSpan(0, 0), "Hello")),
          extensions: [],
          properties: [
            GTProperty(
              span: GTSpan(0, 0),
              doc: None,
              attributes: [],
              name: GTKey(GTSpan(0, 0), "world"),
              descriptor: Object(GTObject(
                span: GTSpan(0, 0),
                name: Named(GTIdentifier(GTSpan(0, 0), "Hi")),
                extensions: [],
                properties: [
                  GTProperty(
                    span: GTSpan(0, 0),
                    doc: None,
                    attributes: [],
                    name: GTKey(GTSpan(0, 0), "world"),
                    descriptor: Object(GTObject(
                      span: GTSpan(0, 0),
                      name: Named(GTIdentifier(GTSpan(0, 0), "HiWorld")),
                      extensions: [],
                      properties: [],
                    )),
                    required: false,
                  ),
                ],
              )),
              required: false,
            ),
          ],
        )
        "#);
    }
}
