use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GtUnion> for GtjUnion {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GtUnion {
        let name = context.claim_name(self.name.clone(), "Union");

        let descriptors =
            context.enter_name_context(GtNamingContextName::Identifier(name.clone()), |context| {
                self.descriptors
                    .iter()
                    .map(|descriptor| {
                        context.enter_name_context(
                            GtNamingContextName::Transitive("Member".into()),
                            |context| descriptor.to_tree_with_context(context),
                        )
                    })
                    .collect()
            });

        GtUnion {
            span: GtSpan::default(),
            doc: None,
            attributes: vec![],
            descriptors,
        }
    }
}

impl GtjTreeConvert<GtDescriptor> for GtjUnion {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GtDescriptor {
        GtDescriptor::Union(self.to_tree_with_context(context))
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

        let descriptor_tree: GtDescriptor = union.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(descriptor_tree, @"
        Union(GtUnion(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          descriptors: [],
        ))
        ");

        let union_tree: GtUnion = union.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(union_tree, @"
        GtUnion(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
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

        let union_tree: GtUnion = union.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(union_tree, @"
        GtUnion(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          descriptors: [
            Primitive(GtPrimitive(
              span: GtSpan(0, 0),
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

        let union_tree: GtUnion = union.to_tree_with_context(&mut context);
        assert_ron_snapshot!(union_tree, @r#"
        GtUnion(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          descriptors: [
            Object(GtObject(
              span: GtSpan(0, 0),
              doc: None,
              attributes: [],
              name: Named(GtIdentifier(GtSpan(0, 0), "RootMember")),
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

        let union_tree: GtUnion = union.to_tree_with_context(&mut context);
        assert_ron_snapshot!(union_tree, @r#"
        GtUnion(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          descriptors: [
            Union(GtUnion(
              span: GtSpan(0, 0),
              doc: None,
              attributes: [],
              descriptors: [
                Object(GtObject(
                  span: GtSpan(0, 0),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(0, 0), "RootMemberMember")),
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
                        name: Named(GtIdentifier(GtSpan(0, 0), "RootMemberMemberWorld")),
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

        let union_tree: GtUnion = union.to_tree_with_context(&mut context);
        assert_ron_snapshot!(union_tree, @r#"
        GtUnion(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          descriptors: [
            Object(GtObject(
              span: GtSpan(0, 0),
              doc: None,
              attributes: [],
              name: Named(GtIdentifier(GtSpan(0, 0), "Hello")),
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

        let union_tree: GtUnion = union.to_tree_with_context(&mut context);
        assert_ron_snapshot!(union_tree, @r#"
        GtUnion(
          span: GtSpan(0, 0),
          doc: None,
          attributes: [],
          descriptors: [
            Union(GtUnion(
              span: GtSpan(0, 0),
              doc: None,
              attributes: [],
              descriptors: [
                Object(GtObject(
                  span: GtSpan(0, 0),
                  doc: None,
                  attributes: [],
                  name: Named(GtIdentifier(GtSpan(0, 0), "HiMember")),
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
                        name: Named(GtIdentifier(GtSpan(0, 0), "HiMemberWorld")),
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
