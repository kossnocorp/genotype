use crate::prelude::internal::*;

impl PYConvert<PYDefinition> for GTAlias {
    fn convert(&self, context: &mut PYConvertContext) -> PYDefinition {
        let doc = self.doc.as_ref().map(|doc| doc.convert(context));

        let name = self.name.convert(context);
        context.push_defined(&name);

        match &self.descriptor {
            GTDescriptor::Object(object) => {
                context.provide_doc(doc);
                PYDefinition::Class(object.convert(context))
            }

            GTDescriptor::Branded(branded) => {
                context.provide_doc(doc);
                PYDefinition::Newtype(branded.convert(context))
            }

            _ => {
                context.create_references_scope();

                let mut descriptor = self.descriptor.convert(context);

                for attribute in self.attributes.iter() {
                    if let PYDescriptor::Union(union) = &mut descriptor {
                        if let Some(assignment) = attribute.get_assigned("discriminator") {
                            if let GTAttributeValue::Literal(literal) = &assignment.value {
                                if let GTLiteralValue::String(value) = &literal.value {
                                    union.discriminator = value.clone().into();
                                    // [TODO] Resolve right now is a mess, instead of resolving in
                                    // convert functions, it should be resolved in the end or by
                                    // the parent.
                                    union.clone().resolve(context);
                                }
                            }
                        }
                    }
                }

                let references = context.pop_references_scope();

                PYDefinition::Alias(PYAlias {
                    doc,
                    name,
                    descriptor,
                    references,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert_alias() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GtFactory::primitive_boolean().into()
            }
            .convert(&mut PYConvertContext::default()),
            @r#"
        Alias(PYAlias(
          doc: None,
          name: PYIdentifier("Name"),
          descriptor: Primitive(Boolean),
          references: [],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_class() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Object(GTObject {
                    span: (0, 0).into(),
                    doc: None,
                    attributes: vec![],
                    name: GTIdentifier::new((0, 0).into(), "Book".into()).into(),
                    extensions: vec![],
                    properties: vec![
                        GTProperty {
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTKey::new((0, 0).into(), "title".into()),
                            descriptor: GtFactory::primitive_string().into(),
                            required: true,
                        },
                        GTProperty {
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GTKey::new((0, 0).into(), "author".into()),
                            descriptor: GtFactory::primitive_string().into(),
                            required: true,
                        }
                    ]
                })
            }
            .convert(&mut PYConvertContext::default()),
            @r#"
        Class(PYClass(
          doc: None,
          name: PYIdentifier("Book"),
          extensions: [],
          properties: [
            PYProperty(
              doc: None,
              name: PYKey("title"),
              descriptor: Primitive(String),
              required: true,
            ),
            PYProperty(
              doc: None,
              name: PYKey("author"),
              descriptor: Primitive(String),
              required: true,
            ),
          ],
          references: [],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_branded() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "UserId".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                descriptor: GtFactory::descriptor(
                    GtFactory::branded("UserId", GtFactory::primitive_string())
                )
            }
            .convert(&mut PYConvertContext::default()),
            @r#"
        Newtype(PYNewtype(
          doc: None,
          name: PYIdentifier("UserId"),
          primitive: String,
        ))
        "#
        );
    }

    #[test]
    fn test_convert_hoisted() {
        let mut context = PYConvertContext::default();

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
                            doc: None,
                            attributes: vec![],
                            name: GTObjectName::Named(GTIdentifier::new(
                                (0, 0).into(),
                                "BookObj".into()
                            )),
                            extensions: vec![],
                            properties: vec![GTProperty {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                name: GTKey::new((0, 0).into(), "author".into()),
                                descriptor: GtFactory::primitive_string().into(),
                                required: true,
                            }]
                        }
                        .into(),
                        GtFactory::primitive_string().into(),
                    ]
                })
            }
            .convert(&mut context),
            @r#"
        Alias(PYAlias(
          doc: None,
          name: PYIdentifier("Book"),
          descriptor: Union(PYUnion(
            descriptors: [
              Reference(PYReference(
                identifier: PYIdentifier("BookObj"),
                forward: true,
              )),
              Primitive(String),
            ],
            discriminator: None,
          )),
          references: [
            PYIdentifier("BookObj"),
          ],
        ))
        "#
        );

        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Class(PYClass(
            doc: None,
            name: PYIdentifier("BookObj"),
            extensions: [],
            properties: [
              PYProperty(
                doc: None,
                name: PYKey("author"),
                descriptor: Primitive(String),
                required: true,
              ),
            ],
            references: [],
          )),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::new(
            Default::default(),
            PyConfig {
                lang: PyConfigLang::new(PYVersion::Legacy),
                ..Default::default()
            },
        );

        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Order".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GtFactory::primitive_string().into(),
            }
            .convert(&mut context),
            @r#"
        Alias(PYAlias(
          doc: None,
          name: PYIdentifier("Name"),
          descriptor: Primitive(String),
          references: [],
        ))
        "#
        );

        assert_eq!(context.as_dependencies(), vec![]);
    }

    #[test]
    fn test_forward_alias() {
        let mut context = PYConvertContext::default();

        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GtFactory::primitive_string().into(),
            }
            .convert(&mut context),
            @r#"
        Alias(PYAlias(
          doc: None,
          name: PYIdentifier("Name"),
          descriptor: Primitive(String),
          references: [],
        ))
        "#
        );

        assert!(context.is_forward_identifier(
            &"Hello".into(),
            &GTIdentifier::new((0, 0).into(), "Hello".into())
        ));
        assert!(!context.is_forward_identifier(
            &"Name".into(),
            &GTIdentifier::new((0, 0).into(), "Name".into())
        ));
    }

    #[test]
    fn test_forward_class() {
        let mut context = PYConvertContext::default();

        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTObject {
                    name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Name".into())),
                    doc: None,
                    attributes: vec![],
                    span: (0, 0).into(),
                    extensions: vec![],
                    properties: vec![],
                }
                .into(),
            }
            .convert(&mut context),
            @r#"
        Class(PYClass(
          doc: None,
          name: PYIdentifier("Name"),
          extensions: [],
          properties: [],
          references: [],
        ))
        "#
        );

        assert!(context.is_forward_identifier(
            &"Hello".into(),
            &GTIdentifier::new((0, 0).into(), "Hello".into())
        ));
        assert!(!context.is_forward_identifier(
            &"Name".into(),
            &GTIdentifier::new((0, 0).into(), "Name".into())
        ));
    }

    #[test]
    fn test_convert_discriminator() {
        assert_ron_snapshot!(
            GTAlias {
                id: GTDefinitionId("module".into(), "Message".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![GTAttribute {
                    span: (0, 0).into(),
                    name: GTAttributeName::new((0, 0).into(), "discriminator".into()),
                    descriptor: Some(GTAttributeDescriptor::Assignment(
                        GTAttributeAssignment::new(
                            (0, 0).into(),
                            GTAttributeValue::Literal(GTLiteral {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                value: GTLiteralValue::String("type".into()),
                            })
                        )
                    ))
                }],
                name: GTIdentifier::new((0, 0).into(), "Message".into()),
                descriptor: GTDescriptor::Union(GTUnion {
                    span: (0, 0).into(),
                    descriptors: vec![
                        GtFactory::reference("Reply").into(),
                        GtFactory::reference("DM").into(),
                    ]
                })
            }
            .convert(&mut PYConvertContext::default()),
            @r#"
        Alias(PYAlias(
          doc: None,
          name: PYIdentifier("Message"),
          descriptor: Union(PYUnion(
            descriptors: [
              Reference(PYReference(
                identifier: PYIdentifier("Reply"),
                forward: true,
              )),
              Reference(PYReference(
                identifier: PYIdentifier("DM"),
                forward: true,
              )),
            ],
            discriminator: Some("type"),
          )),
          references: [
            PYIdentifier("Reply"),
            PYIdentifier("DM"),
          ],
        ))
        "#
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
                descriptor: GtFactory::primitive_boolean().into(),
            }
            .convert(&mut PYConvertContext::default()),
            @r#"
        Alias(PYAlias(
          doc: Some(PYDoc("Hello, world!")),
          name: PYIdentifier("Name"),
          descriptor: Primitive(Boolean),
          references: [],
        ))
        "#
        );
    }
}
