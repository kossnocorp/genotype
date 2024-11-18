use genotype_lang_rs_tree::*;
use genotype_parser::*;
use miette::Result;

use crate::{
    context::{naming::RSContextParent, RSConvertContext},
    convert::RSConvert,
};

impl RSConvert<RSDefinition> for GTAlias {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSDefinition> {
        let doc = if let Some(doc) = &self.doc {
            Some(doc.convert(context)?)
        } else {
            None
        };

        let name = self.name.convert(context)?;
        context.push_defined(&name);

        let definition = match &self.descriptor {
            GTDescriptor::Object(object) => {
                context.provide_definition_id(self.id.clone());
                context.provide_doc(doc);
                RSDefinition::Struct(object.convert(context)?)
            }

            GTDescriptor::Union(union) => {
                context.provide_definition_id(self.id.clone());
                context.provide_doc(doc);
                RSDefinition::Enum(union.convert(context)?)
            }

            _ => {
                context.enter_parent(RSContextParent::Alias(name.clone()));

                let descriptor = self.descriptor.convert(context)?;
                let alias = RSDefinition::Alias(RSAlias {
                    id: self.id.clone(),
                    doc,
                    name,
                    descriptor,
                });

                context.exit_parent();
                alias
            }
        };

        Ok(definition)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

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
                attributes: vec![
                    "derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)"
                        .into()
                ],
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
                id: GTDefinitionId("module".into(), "Union".into()),
                doc: None,
                attributes: vec![
                    "derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)"
                        .into(),
                    r#"serde(untagged)"#.into(),
                ],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        name: "BookObj".into(),
                        attributes: vec![],
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference::new(
                                "BookObj".into(),
                                GTDefinitionId("module".into(), "BookObj".into())
                            )
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
                attributes: vec![
                    "derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)"
                        .into(),
                ],
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
    fn test_convert_discriminator() {
        assert_eq!(
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
                            GTAttributeValue::Literal(GTLiteral::String(
                                (0, 0).into(),
                                "type".into()
                            ))
                        )
                    ))
                }],
                name: GTIdentifier::new((0, 0).into(), "Message".into()),
                descriptor: GTDescriptor::Union(GTUnion {
                    span: (0, 0).into(),
                    descriptors: vec![
                        GTIdentifier((0, 0).into(), "Reply".into()).into(),
                        GTIdentifier((0, 0).into(), "DM".into()).into(),
                    ]
                })
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSDefinition::Enum(RSEnum {
                id: GTDefinitionId("module".into(), "Message".into()),
                doc: None,
                name: "Message".into(),
                attributes: vec![],
                variants: vec![]
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
