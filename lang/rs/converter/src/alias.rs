use genotype_lang_rs_tree::*;
use genotype_parser::*;

use crate::{
    context::{naming::RSContextParent, RSConvertContext},
    convert::RSConvert,
};

impl RSConvert<RSDefinition> for GTAlias {
    fn convert(&self, context: &mut RSConvertContext) -> RSDefinition {
        let doc = self.doc.as_ref().map(|doc| doc.convert(context));

        let name = self.name.convert(context);
        context.push_defined(&name);

        let definition = match &self.descriptor {
            GTDescriptor::Object(object) => {
                context.provide_doc(doc);
                RSDefinition::Struct(object.convert(context))
            }

            GTDescriptor::Union(union) => {
                context.provide_doc(doc);
                RSDefinition::Enum(union.convert(context))
            }

            _ => {
                context.enter_parent(RSContextParent::Alias(name.clone()));
                let mut descriptor = self.descriptor.convert(context);

                // for attribute in self.attributes.iter() {
                //     if let RSDescriptor::Union(union) = &mut descriptor {
                //         if let Some(assignment) = attribute.get_assigned("discriminator") {
                //             if let GTAttributeValue::Literal(GTLiteral::String(_, value)) =
                //                 &assignment.value
                //             {
                //                 union.discriminator = value.clone().into();
                //                 // [TODO] Resolve right now is a mess, instead of resolving in
                //                 // convert functions, it should be resolved in the end or by
                //                 // the parent.
                //                 union.clone().resolve(context);
                //             }
                //         }
                //     }
                // }

                let alias = RSDefinition::Alias(RSAlias {
                    doc,
                    name,
                    descriptor,
                });

                context.exit_parent();
                alias
            }
        };

        definition
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
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut RSConvertContext::default()),
            RSDefinition::Alias(RSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
            }),
        );
    }

    #[test]
    fn test_convert_class() {
        assert_eq!(
            GTAlias {
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
            .convert(&mut RSConvertContext::default()),
            RSDefinition::Struct(RSStruct {
                doc: None,
                attributes: vec![],
                name: "Book".into(),
                extensions: vec![],
                properties: vec![
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "title".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    },
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "author".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    }
                ],
            }),
        );
    }

    #[test]
    #[ignore = "WIP"]
    fn test_convert_hoisted() {
        let mut context = RSConvertContext::default();
        assert_eq!(
            GTAlias {
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
            .convert(&mut context),
            RSDefinition::Enum(RSEnum {
                doc: None,
                attributes: vec![],
                name: "Book".into(),
                variants: vec![]
            })
        );
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![RSDefinition::Struct(RSStruct {
                doc: None,
                attributes: vec![],
                name: "BookObj".into(),
                extensions: vec![],
                properties: vec![RSProperty {
                    doc: None,
                    attributes: vec![],
                    name: "author".into(),
                    descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                }],
            })]
        );
    }

    #[test]
    #[ignore = "WIP"]
    fn test_convert_discriminator() {
        assert_eq!(
            GTAlias {
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
            .convert(&mut RSConvertContext::default()),
            RSDefinition::Enum(RSEnum {
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
                span: (0, 0).into(),
                doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut RSConvertContext::default()),
            RSDefinition::Alias(RSAlias {
                doc: Some("Hello, world!".into()),
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
            }),
        );
    }
}
