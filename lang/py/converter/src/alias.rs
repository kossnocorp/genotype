use genotype_lang_py_tree::*;
use genotype_parser::*;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYDefinition> for GTAlias {
    fn convert(&self, context: &mut PYConvertContext) -> PYDefinition {
        match &self.descriptor {
            GTDescriptor::Object(object) => PYDefinition::Class(object.convert(context)),

            _ => {
                let name = self.name.convert(context);
                context.push_defined(&name);

                let mut descriptor = self.descriptor.convert(context);

                for attribute in self.attributes.iter() {
                    if let PYDescriptor::Union(union) = &mut descriptor {
                        if let Some(assignment) = attribute.get_assigned("discriminator") {
                            if let GTAttributeValue::Literal(GTLiteral::String(_, value)) =
                                &assignment.value
                            {
                                union.discriminator = value.clone().into();
                                // [TODO] Resolve right now is a mess, instead of resolving in
                                // cconver functions, it should be resolved in the end or by
                                // the parent.
                                union.clone().resolve(context);
                            }
                        }
                    }
                }

                PYDefinition::Alias(
                    PYAlias {
                        doc: self.doc.as_ref().map(|doc| doc.convert(context)),
                        name,
                        descriptor,
                    }
                    .resolve(context),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_config::{PYLangConfig, PYVersion};
    use genotype_lang_py_tree::*;
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
            .convert(&mut PYConvertContext::default()),
            PYDefinition::Alias(PYAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
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
            .convert(&mut PYConvertContext::default()),
            PYDefinition::Class(PYClass {
                doc: None,
                name: "Book".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        name: "title".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true,
                    },
                    PYProperty {
                        name: "author".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true,
                    }
                ]
            }),
        );
    }

    #[test]
    fn test_convert_hoisted() {
        let mut context = PYConvertContext::default();
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
            PYDefinition::Alias(PYAlias {
                doc: None,
                name: "Book".into(),
                descriptor: PYUnion {
                    descriptors: vec![
                        PYReference::new("BookObj".into(), true).into(),
                        PYPrimitive::String.into(),
                    ],
                    discriminator: None
                }
                .into(),
            })
        );
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![PYDefinition::Class(PYClass {
                doc: None,
                name: "BookObj".into(),
                extensions: vec![],
                properties: vec![PYProperty {
                    name: "author".into(),
                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                    required: true,
                }]
            })]
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context =
            PYConvertContext::new(Default::default(), PYLangConfig::new(PYVersion::Legacy));
        assert_eq!(
            GTAlias {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut context),
            PYDefinition::Alias(PYAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PYPrimitive::String.into(),
            })
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(PYDependency::Typing, "TypeAlias".into()),]
        );
    }

    #[test]
    fn test_forward() {
        let mut context = PYConvertContext::default();
        assert_eq!(
            GTAlias {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut context),
            PYDefinition::Alias(PYAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PYPrimitive::String.into(),
            })
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
        assert_eq!(
            GTAlias {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![GTAttribute {
                    span: (0, 0).into(),
                    name: GTAttributeName::new((0, 0).into(), "discriminator".into()),
                    descriptor: Some(GTAttributeDescriptor::Assigment(
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
            .convert(&mut PYConvertContext::default()),
            PYDefinition::Alias(PYAlias {
                doc: None,
                name: "Message".into(),
                descriptor: PYUnion {
                    descriptors: vec![
                        PYReference::new("Reply".into(), true).into(),
                        PYReference::new("DM".into(), true).into(),
                    ],
                    discriminator: Some("type".into())
                }
                .into()
            }),
        );
    }

    #[test]
    fn test_convert_doc() {
        assert_eq!(
            GTAlias {
                span: (0, 0).into(),
                doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut PYConvertContext::default()),
            PYDefinition::Alias(PYAlias {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
            }),
        );
    }
}
