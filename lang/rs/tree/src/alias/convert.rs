use crate::prelude::internal::*;

impl RSConvert<RSDefinition> for GTAlias {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSDefinition> {
        let doc = if let Some(doc) = &self.doc {
            Some(doc.convert(context)?)
        } else {
            None
        };

        let name = self.name.convert(context)?;
        context.push_defined(&name);
        context.enter_parent(RSContextParent::Alias(name.clone()));

        let definition = match &self.descriptor {
            GTDescriptor::Object(object) => {
                context.provide_definition_id(self.id.clone());
                context.provide_doc(doc);
                RSDefinition::Struct(object.convert(context)?)
            }

            GTDescriptor::Branded(branded) => {
                context.provide_definition_id(self.id.clone());
                context.provide_doc(doc);
                RSDefinition::Struct(branded.convert(context)?)
            }

            GTDescriptor::Union(union) => {
                context.provide_definition_id(self.id.clone());
                context.provide_doc(doc);
                RSDefinition::Enum(union.convert(context)?)
            }

            _ => {
                let descriptor = self.descriptor.convert(context)?;
                let alias = RSDefinition::Alias(RSAlias {
                    id: self.id.clone(),
                    doc,
                    name,
                    descriptor,
                });

                alias
            }
        };

        context.exit_parent();
        Ok(definition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSDefinition::Alias(RSAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
            }),
        );
    }

    #[test]
    fn test_convert_struct() {
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
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSDefinition::Struct(RSStruct {
                id: GTDefinitionId("module".into(), "Book".into()),
                doc: None,
                attributes: vec!["derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into()],
                name: "Book".into(),
                fields: vec![
                    RSField {
                        doc: None,
                        attributes: vec![],
                        name: "title".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    },
                    RSField {
                        doc: None,
                        attributes: vec![],
                        name: "author".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    }
                ]
                .into(),
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
                    name: GTIdentifier((0, 0).into(), "BookId".into()),
                    primitive: GTPrimitive::Int32((0, 0).into())
                })
                .into(),
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSDefinition::Struct(RSStruct {
                id: GTDefinitionId("module".into(), "BookId".into()),
                doc: None,
                attributes: vec!["derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into()],
                name: "BookId".into(),
                fields: RSStructFields::Tuple(vec![RSDescriptor::Primitive(RSPrimitive::Int32),])
                    .into(),
            }),
        );
    }

    #[test]
    fn test_convert_hoisted() {
        let mut context = RSConvertContext::empty("module".into());
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
                                descriptor: GTPrimitive::String((0, 0).into()).into(),
                                required: true,
                            }]
                        }
                        .into(),
                        GTPrimitive::String((0, 0).into()).into(),
                    ]
                })
            }
            .convert(&mut context)
            .unwrap(),
            RSDefinition::Enum(RSEnum {
                id: GTDefinitionId("module".into(), "Book".into()),
                doc: None,
                attributes: vec![
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into(),
                    r#"serde(untagged)"#.into(),
                ],
                name: "Book".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        name: "BookObj".into(),
                        attributes: vec![],
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference {
                                id: GTReferenceId("module".into(), (0, 0).into()),
                                identifier: "BookObj".into(),
                                definition_id: GTDefinitionId("module".into(), "BookObj".into())
                            }
                            .into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        name: "String".into(),
                        attributes: vec![],
                        descriptor: RSEnumVariantDescriptor::Descriptor(RSPrimitive::String.into()),
                    }
                ]
            })
        );
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![RSDefinition::Struct(RSStruct {
                id: GTDefinitionId("module".into(), "BookObj".into()),
                doc: None,
                attributes: vec!["derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into(),],
                name: "BookObj".into(),
                fields: vec![RSField {
                    doc: None,
                    attributes: vec![],
                    name: "author".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                }]
                .into(),
            })]
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
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSDefinition::Alias(RSAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: Some("Hello, world!".into()),
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
            }),
        );
    }
}
