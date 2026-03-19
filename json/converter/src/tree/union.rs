use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTUnion> for GtjUnion {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTUnion {
        let name = context.claim_name(self.name.clone(), "Union");

        let descriptors =
            context.enter_name_context(GTNamingContextName::Identifier(name.clone()), |context| {
                self.descriptors
                    .iter()
                    .map(|descriptor| {
                        context.enter_name_context(
                            GTNamingContextName::Transitive("Member".into()),
                            |context| descriptor.to_tree_with_context(context),
                        )
                    })
                    .collect()
            });

        GTUnion {
            span: GTSpan::default(),
            descriptors,
        }
    }
}

impl GtjTreeConvert<GTDescriptor> for GtjUnion {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTDescriptor {
        GTDescriptor::Union(self.to_tree_with_context(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
            descriptors: vec![],
            name: None,
            doc: None,
        };

        let descriptor_tree: GTDescriptor = union.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @"
        Union(GTUnion(
          span: GTSpan(0, 0),
          descriptors: [],
        ))
        ");

        let union_tree: GTUnion = union.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(union_tree, @"
        GTUnion(
          span: GTSpan(0, 0),
          descriptors: [],
        )
        ");
    }

    #[test]
    fn test_convert_descriptors() {
        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
            descriptors: vec![GtjAny::GtjNumber(GtjNumber {
                name: None,
                doc: None,
                r#type: GtjNumberTypeNumber,
            })],
            name: None,
            doc: None,
        };

        let union_tree: GTUnion = union.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(union_tree, @"
        GTUnion(
          span: GTSpan(0, 0),
          descriptors: [
            Primitive(GTPrimitive(
              span: GTSpan(0, 0),
              kind: Number,
              doc: None,
              attributes: [],
            )),
          ],
        )
        ");
    }

    #[test]
    fn test_convert_naming_unnamed() {
        let mut context = GtjTreeConvertContext::new();

        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
            descriptors: vec![GtjAny::GtjObject(GtjObject {
                r#type: GtjObjectTypeObject,
                name: None,
                doc: None,
                properties: vec![],
            })],
            name: None,
            doc: None,
        };

        let union_tree: GTUnion = union.to_tree_with_context(&mut context);
        assert_ron_snapshot!(union_tree, @r#"
        GTUnion(
          span: GTSpan(0, 0),
          descriptors: [
            Object(GTObject(
              span: GTSpan(0, 0),
              name: Named(GTIdentifier(GTSpan(0, 0), "RootMember")),
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

        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
            descriptors: vec![GtjAny::GtjUnion(GtjUnion {
                r#type: GtjUnionTypeUnion,
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

        let union_tree: GTUnion = union.to_tree_with_context(&mut context);
        assert_ron_snapshot!(union_tree, @r#"
        GTUnion(
          span: GTSpan(0, 0),
          descriptors: [
            Union(GTUnion(
              span: GTSpan(0, 0),
              descriptors: [
                Object(GTObject(
                  span: GTSpan(0, 0),
                  name: Named(GTIdentifier(GTSpan(0, 0), "RootMemberMember")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(0, 0),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(0, 0), "world"),
                      descriptor: Object(GTObject(
                        span: GTSpan(0, 0),
                        name: Named(GTIdentifier(GTSpan(0, 0), "RootMemberMemberWorld")),
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

        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
            descriptors: vec![GtjAny::GtjObject(GtjObject {
                r#type: GtjObjectTypeObject,
                name: Some("Hello".into()),
                doc: None,
                properties: vec![],
            })],
            name: Some("World".into()),
            doc: None,
        };

        let union_tree: GTUnion = union.to_tree_with_context(&mut context);
        assert_ron_snapshot!(union_tree, @r#"
        GTUnion(
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

        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
            descriptors: vec![GtjAny::GtjUnion(GtjUnion {
                r#type: GtjUnionTypeUnion,
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

        let union_tree: GTUnion = union.to_tree_with_context(&mut context);
        assert_ron_snapshot!(union_tree, @r#"
        GTUnion(
          span: GTSpan(0, 0),
          descriptors: [
            Union(GTUnion(
              span: GTSpan(0, 0),
              descriptors: [
                Object(GTObject(
                  span: GTSpan(0, 0),
                  name: Named(GTIdentifier(GTSpan(0, 0), "HiMember")),
                  extensions: [],
                  properties: [
                    GTProperty(
                      span: GTSpan(0, 0),
                      doc: None,
                      attributes: [],
                      name: GTKey(GTSpan(0, 0), "world"),
                      descriptor: Object(GTObject(
                        span: GTSpan(0, 0),
                        name: Named(GTIdentifier(GTSpan(0, 0), "HiMemberWorld")),
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
