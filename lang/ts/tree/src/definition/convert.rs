use crate::prelude::internal::*;

impl TSConvert<TSDefinition> for GTAlias {
    fn convert(&self, context: &mut TSConvertContext) -> TSDefinition {
        let doc = self.doc.as_ref().map(|d| d.convert(context));
        let name = self.name.convert(context);

        match &self.descriptor {
            GTDescriptor::Branded(branded) => {
                context.provide_doc(doc);
                TSDefinition::Branded(branded.convert(context))
            }

            GTDescriptor::Object(object) => TSDefinition::Interface(TSInterface {
                doc,
                name,
                extensions: object
                    .extensions
                    .iter()
                    .map(|e| e.convert(context))
                    .collect(),
                properties: object
                    .properties
                    .iter()
                    .map(|p| p.convert(context))
                    .collect(),
            }),

            _ => TSDefinition::Alias(TSAlias {
                doc,
                name,
                descriptor: self.descriptor.convert(context),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_alias() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut Default::default()),
            @r#"
        Alias(TSAlias(
          doc: None,
          name: TSIdentifier("Name"),
          descriptor: Primitive(Boolean),
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_interface() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Object(GTObject {
                    span: (0, 0).into(),
                    name: GTIdentifier::new((0, 0).into(), "Book".into()).into(),
                    extensions: vec![],
                    properties: vec![
                        GTProperty {
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTKey::new((0, 0).into(), "title".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                            required: true,
                        },
                        GTProperty {
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTKey::new((0, 0).into(), "author".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                            required: true,
                        }
                    ]
                })
            }
            .convert(&mut Default::default()),
            @r#"
        Interface(TSInterface(
          doc: None,
          name: TSIdentifier("Book"),
          extensions: [],
          properties: [
            TSProperty(
              doc: None,
              name: TSKey("title"),
              descriptor: Primitive(String),
              required: true,
            ),
            TSProperty(
              doc: None,
              name: TSKey("author"),
              descriptor: Primitive(String),
              required: true,
            ),
          ],
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_branded() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "BookId".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "BookId".into()),
                descriptor: GTDescriptor::Branded(GTBranded {
                    span: (0, 0).into(),
                    id: GTDefinitionId("module".into(), "BookId".into()),
                    name: GTIdentifier::new((0, 0).into(), "BookId".into()),
                    primitive: GTPrimitive::String((0, 0).into()).into(),
                })
            }
            .convert(&mut Default::default()),
            @r#"
        Branded(TSBranded(
          doc: None,
          name: TSIdentifier("BookId"),
          primitive: String,
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_extensions() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Object(GTObject {
                    span: (0, 0).into(),
                    name: GTIdentifier::new((0, 0).into(), "Book".into()).into(),
                    extensions: vec![GTExtension {
                        span: (0, 0).into(),
                        reference: GTReference {
                            span: (0, 0).into(),
                            id: GTReferenceId("module".into(), (0, 0).into()),
                            definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                                "module".into(),
                                "Good".into()
                            )),
                            identifier: GTIdentifier::new((0, 0).into(), "Good".into())
                        }
                        .into()
                    }],
                    properties: vec![GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "author".into()),
                        descriptor: GTPrimitive::String((0, 0).into()).into(),
                        required: true,
                    }]
                })
            }
            .convert(&mut Default::default()),
            @r#"
        Interface(TSInterface(
          doc: None,
          name: TSIdentifier("Book"),
          extensions: [
            TSExtension(
              reference: TSReference(TSIdentifier("Good")),
            ),
          ],
          properties: [
            TSProperty(
              doc: None,
              name: TSKey("author"),
              descriptor: Primitive(String),
              required: true,
            ),
          ],
        ))
        "#,
        );

        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Union(GTUnion {
                    span: (0, 0).into(),
                    descriptors: vec![
                        GTObject {
                            span: (0, 0).into(),
                            name: GTObjectName::Alias(
                                GTIdentifier::new((0, 0).into(), "BookAuthorObj".into()),
                                GTObjectNameParent::Alias(GTIdentifier::new(
                                    (0, 0).into(),
                                    "BookAuthor".into()
                                ))
                            ),
                            extensions: vec![GTExtension {
                                span: (0, 0).into(),
                                reference: GTReference {
                                    span: (0, 0).into(),
                                    id: GTReferenceId("module".into(), (0, 0).into()),
                                    definition_id: GTReferenceDefinitionId::Resolved(
                                        GTDefinitionId("module".into(), "Good".into())
                                    ),
                                    identifier: GTIdentifier::new((0, 0).into(), "Good".into())
                                }
                                .into()
                            }],
                            properties: vec![GTProperty {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                name: GTKey::new((0, 0).into(), "author".into()),
                                descriptor: GTPrimitive::String((0, 0).into()).into(),
                                required: true,
                            }]
                        }
                        .into(),
                        GTPrimitive::String((0, 0).into()).into(),
                    ]
                })
            }
            .convert(&mut Default::default()),
            @r#"
        Alias(TSAlias(
          doc: None,
          name: TSIdentifier("Book"),
          descriptor: Union(TSUnion(
            descriptors: [
              Intersection(TSIntersection(
                descriptors: [
                  Object(TSObject(
                    properties: [
                      TSProperty(
                        doc: None,
                        name: TSKey("author"),
                        descriptor: Primitive(String),
                        required: true,
                      ),
                    ],
                  )),
                  Reference(TSReference(TSIdentifier("Good"))),
                ],
              )),
              Primitive(String),
            ],
          )),
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_doc_interface() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Object(GTObject {
                    span: (0, 0).into(),
                    name: GTIdentifier::new((0, 0).into(), "Book".into()).into(),
                    extensions: vec![],
                    properties: vec![]
                })
            }
            .convert(&mut Default::default()),
            @r#"
        Interface(TSInterface(
          doc: Some(TSDoc("Hello, world!")),
          name: TSIdentifier("Book"),
          extensions: [],
          properties: [],
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_doc_alias() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut Default::default()),
            @r#"
        Alias(TSAlias(
          doc: Some(TSDoc("Hello, world!")),
          name: TSIdentifier("Name"),
          descriptor: Primitive(Boolean),
        ))
        "#,
        );
    }

    #[test]
    fn test_convert_doc_branded() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "BookId".into()),
                span: (0, 0).into(),
                doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "BookId".into()),
                descriptor: GTDescriptor::Branded(GTBranded {
                    span: (0, 0).into(),
                    id: GTDefinitionId("module".into(), "BookId".into()),
                    name: GTIdentifier::new((0, 0).into(), "BookId".into()),
                    primitive: GTPrimitive::String((0, 0).into()).into(),
                })
            }
            .convert(&mut Default::default()),
            @r#"
        Branded(TSBranded(
          doc: Some(TSDoc("Hello, world!")),
          name: TSIdentifier("BookId"),
          primitive: String,
        ))
        "#,
        );
    }
}
