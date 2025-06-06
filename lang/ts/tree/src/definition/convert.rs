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
    use pretty_assertions::assert_eq;
    use std::vec;

    #[test]
    fn test_convert_alias() {
        assert_eq!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut Default::default()),
            TSDefinition::Alias(TSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::Boolean),
            }),
        );
    }

    #[test]
    fn test_convert_interface() {
        assert_eq!(
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
            TSDefinition::Interface(TSInterface {
                doc: None,
                name: "Book".into(),
                extensions: vec![],
                properties: vec![
                    TSProperty {
                        doc: None,
                        name: "title".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true,
                    },
                    TSProperty {
                        doc: None,
                        name: "author".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true,
                    }
                ]
            }),
        );
    }

    #[test]
    fn test_convert_branded() {
        assert_eq!(
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
            TSDefinition::Branded(TSBranded {
                doc: None,
                name: "BookId".into(),
                primitive: TSPrimitive::String,
            }),
        );
    }

    #[test]
    fn test_convert_extensions() {
        assert_eq!(
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
            TSDefinition::Interface(TSInterface {
                doc: None,
                name: "Book".into(),
                extensions: vec!["Good".into()],
                properties: vec![TSProperty {
                    doc: None,
                    name: "author".into(),
                    descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                    required: true,
                }]
            }),
        );

        assert_eq!(
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
            TSDefinition::Alias(TSAlias {
                doc: None,
                name: "Book".into(),
                descriptor: TSUnion {
                    descriptors: vec![
                        TSIntersection {
                            descriptors: vec![
                                TSObject {
                                    properties: vec![TSProperty {
                                        doc: None,
                                        name: "author".into(),
                                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                                        required: true,
                                    }]
                                }
                                .into(),
                                "Good".into()
                            ],
                        }
                        .into(),
                        TSPrimitive::String.into(),
                    ]
                }
                .into(),
            }),
        );
    }

    #[test]
    fn test_convert_doc_interface() {
        assert_eq!(
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
            TSDefinition::Interface(TSInterface {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "Book".into(),
                extensions: vec![],
                properties: vec![]
            }),
        );
    }

    #[test]
    fn test_convert_doc_alias() {
        assert_eq!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut Default::default()),
            TSDefinition::Alias(TSAlias {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::Boolean),
            }),
        );
    }

    #[test]
    fn test_convert_doc_branded() {
        assert_eq!(
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
            TSDefinition::Branded(TSBranded {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "BookId".into(),
                primitive: TSPrimitive::String,
            }),
        );
    }
}
