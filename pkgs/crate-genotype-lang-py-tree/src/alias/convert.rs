use crate::prelude::internal::*;

impl PyConvert<PyDefinition> for GtAlias {
    fn convert(&self, context: &mut PyConvertContext) -> PyDefinition {
        let doc = self.doc.as_ref().map(|doc| doc.convert(context));

        let name = self.name.convert(context);
        context.push_defined(&name);

        match &self.descriptor {
            GtDescriptor::Object(object) => {
                context.provide_doc(doc);
                PyDefinition::Class(object.convert(context))
            }

            GtDescriptor::Branded(branded) => {
                context.provide_doc(doc);
                PyDefinition::Newtype(branded.convert(context))
            }

            _ => {
                context.create_references_scope();

                let mut descriptor = self.descriptor.convert(context);

                for attribute in self.attributes.iter() {
                    if let PyDescriptor::Union(union) = &mut descriptor {
                        if let Some(assignment) = attribute.get_assigned("discriminator") {
                            if let GtAttributeValue::Literal(literal) = &assignment.value {
                                if let GtLiteralValue::String(value) = &literal.value {
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

                PyDefinition::Alias(PyAlias {
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
            GtAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: Gt::primitive_boolean().into()
            }
            .convert(&mut PyConvertContext::default()),
            @r#"
        Alias(PyAlias(
          doc: None,
          name: PyIdentifier("Name"),
          descriptor: Primitive(Boolean),
          references: [],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_class() {
        assert_ron_snapshot!(
            GtAlias {
                id: GtDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GtDescriptor::Object(GtObject {
                    span: (0, 0).into(),
                    doc: None,
                    attributes: vec![],
                    name: GtIdentifier::new((0, 0).into(), "Book".into()).into(),
                    extensions: vec![],
                    properties: vec![
                        GtProperty {
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GtKey::new((0, 0).into(), "title".into()),
                            descriptor: Gt::primitive_string().into(),
                            required: true,
                        },
                        GtProperty {
                            span: (0, 0).into(),
                            doc: None,
                            attributes: vec![],
                            name: GtKey::new((0, 0).into(), "author".into()),
                            descriptor: Gt::primitive_string().into(),
                            required: true,
                        }
                    ]
                })
            }
            .convert(&mut PyConvertContext::default()),
            @r#"
        Class(PyClass(
          doc: None,
          name: PyIdentifier("Book"),
          extensions: [],
          properties: [
            PyProperty(
              doc: None,
              name: PyKey("title"),
              descriptor: Primitive(String),
              required: true,
            ),
            PyProperty(
              doc: None,
              name: PyKey("author"),
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
            GtAlias {
                id: GtDefinitionId("module".into(), "UserId".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "UserId".into()),
                descriptor: Gt::descriptor(
                    Gt::branded("UserId", Gt::primitive_string())
                )
            }
            .convert(&mut PyConvertContext::default()),
            @r#"
        Newtype(PyNewtype(
          doc: None,
          name: PyIdentifier("UserId"),
          primitive: String,
        ))
        "#
        );
    }

    #[test]
    fn test_convert_hoisted() {
        let mut context = PyConvertContext::default();

        assert_ron_snapshot!(
            GtAlias {
                id: GtDefinitionId("module".into(), "Book".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: Gt::descriptor(Gt::union(vec_into![
                    Gt::object("BookObj", vec![
                        Gt::property("author", Gt::primitive_string())
                    ]),
                    Gt::primitive_string()
                ])),
            }
            .convert(&mut context),
            @r#"
        Alias(PyAlias(
          doc: None,
          name: PyIdentifier("Book"),
          descriptor: Union(PyUnion(
            descriptors: [
              Reference(PyReference(
                identifier: PyIdentifier("BookObj"),
                forward: true,
              )),
              Primitive(String),
            ],
            discriminator: None,
          )),
          references: [
            PyIdentifier("BookObj"),
          ],
        ))
        "#
        );

        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Class(PyClass(
            doc: None,
            name: PyIdentifier("BookObj"),
            extensions: [],
            properties: [
              PyProperty(
                doc: None,
                name: PyKey("author"),
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
        let mut context = PyConvertContext::new(
            Default::default(),
            PyConfig {
                lang: PyConfigLang::new(PyVersion::Legacy),
                ..Default::default()
            },
        );

        assert_ron_snapshot!(
            GtAlias {
                id: GtDefinitionId("module".into(), "Order".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: Gt::primitive_string().into(),
            }
            .convert(&mut context),
            @r#"
        Alias(PyAlias(
          doc: None,
          name: PyIdentifier("Name"),
          descriptor: Primitive(String),
          references: [],
        ))
        "#
        );

        assert_ron_snapshot!(
            context.imports(),
            @"[]"
        )
    }

    #[test]
    fn test_forward_alias() {
        let mut context = PyConvertContext::default();

        assert_ron_snapshot!(
            GtAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: Gt::primitive_string().into(),
            }
            .convert(&mut context),
            @r#"
        Alias(PyAlias(
          doc: None,
          name: PyIdentifier("Name"),
          descriptor: Primitive(String),
          references: [],
        ))
        "#
        );

        assert!(context.is_forward_identifier(
            &"Hello".into(),
            &GtIdentifier::new((0, 0).into(), "Hello".into())
        ));
        assert!(!context.is_forward_identifier(
            &"Name".into(),
            &GtIdentifier::new((0, 0).into(), "Name".into())
        ));
    }

    #[test]
    fn test_forward_class() {
        let mut context = PyConvertContext::default();

        assert_ron_snapshot!(
            GtAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GtObject {
                    name: GtObjectName::Named(GtIdentifier::new((0, 0).into(), "Name".into())),
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
        Class(PyClass(
          doc: None,
          name: PyIdentifier("Name"),
          extensions: [],
          properties: [],
          references: [],
        ))
        "#
        );

        assert!(context.is_forward_identifier(
            &"Hello".into(),
            &GtIdentifier::new((0, 0).into(), "Hello".into())
        ));
        assert!(!context.is_forward_identifier(
            &"Name".into(),
            &GtIdentifier::new((0, 0).into(), "Name".into())
        ));
    }

    #[test]
    fn test_convert_discriminator() {
        assert_ron_snapshot!(
            GtAlias {
                id: GtDefinitionId("module".into(), "Message".into()),
                span: (0, 0).into(),
                doc: None,
                attributes: vec![GtAttribute {
                    span: (0, 0).into(),
                    name: GtAttributeName::new((0, 0).into(), "discriminator".into()),
                    descriptor: Some(GtAttributeDescriptor::Assignment(
                        GtAttributeAssignment::new(
                            (0, 0).into(),
                            GtAttributeValue::Literal(GtLiteral {
                                span: (0, 0).into(),
                                doc: None,
                                attributes: vec![],
                                value: GtLiteralValue::String("type".into()),
                            })
                        )
                    ))
                }],
                name: GtIdentifier::new((0, 0).into(), "Message".into()),
                descriptor: Gt::descriptor(Gt::union(vec_into![
                    Gt::reference("Reply"),
                    Gt::reference("DM")
                ])),
            }
            .convert(&mut PyConvertContext::default()),
            @r#"
        Alias(PyAlias(
          doc: None,
          name: PyIdentifier("Message"),
          descriptor: Union(PyUnion(
            descriptors: [
              Reference(PyReference(
                identifier: PyIdentifier("Reply"),
                forward: true,
              )),
              Reference(PyReference(
                identifier: PyIdentifier("DM"),
                forward: true,
              )),
            ],
            discriminator: Some("type"),
          )),
          references: [
            PyIdentifier("Reply"),
            PyIdentifier("DM"),
          ],
        ))
        "#
        );
    }

    #[test]
    fn test_convert_doc_alias() {
        assert_ron_snapshot!(
            GtAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                span: (0, 0).into(),
                doc: Some(GtDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: Gt::primitive_boolean().into(),
            }
            .convert(&mut PyConvertContext::default()),
            @r#"
        Alias(PyAlias(
          doc: Some(PyDoc("Hello, world!")),
          name: PyIdentifier("Name"),
          descriptor: Primitive(Boolean),
          references: [],
        ))
        "#
        );
    }
}
