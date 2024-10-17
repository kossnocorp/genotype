use genotype_lang_py_tree::*;
use genotype_parser::*;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYDefinition> for GTAlias {
    fn convert(&self, context: &mut PYConvertContext) -> PYDefinition {
        match &self.descriptor {
            GTDescriptor::Object(object) => PYDefinition::Class(object.convert(context)),

            _ => PYDefinition::Alias(PYAlias {
                name: self.name.convert(context),
                descriptor: self.descriptor.convert(context),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    use crate::resolve::PYConvertResolve;

    use super::*;

    #[test]
    fn test_convert_alias() {
        assert_eq!(
            GTAlias {
                span: (0, 0).into(),
                doc: None,
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut PYConvertContext::default()),
            PYDefinition::Alias(PYAlias {
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
                name: GTIdentifier::new((0, 0).into(), "Book".into()),
                descriptor: GTDescriptor::Object(GTObject {
                    span: (0, 0).into(),
                    name: GTIdentifier::new((0, 0).into(), "Book".into()).into(),
                    extensions: vec![],
                    properties: vec![
                        GTProperty {
                            span: (0, 0).into(),
                            doc: None,
                            name: GTKey::new((0, 0).into(), "title".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                            required: true,
                        },
                        GTProperty {
                            span: (0, 0).into(),
                            doc: None,
                            name: GTKey::new((0, 0).into(), "author".into()),
                            descriptor: GTPrimitive::String((0, 0).into()).into(),
                            required: true,
                        }
                    ]
                })
            }
            .convert(&mut PYConvertContext::default()),
            PYDefinition::Class(PYClass {
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
        let hoisted = Arc::new(Mutex::new(vec![]));
        let mut context = {
            let hoisted = Arc::clone(&hoisted);
            PYConvertContext::new(
                PYConvertResolve::new(),
                Box::new(move |definition| {
                    let mut hoisted = hoisted.lock().unwrap();
                    hoisted.push(definition);
                }),
            )
        };
        assert_eq!(
            GTAlias {
                span: (0, 0).into(),
                doc: None,
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
                name: "Book".into(),
                descriptor: PYUnion {
                    descriptors: vec![
                        PYReference::new("BookObj".into(), true).into(),
                        PYPrimitive::String.into(),
                    ]
                }
                .into(),
            })
        );
        assert_eq!(
            hoisted.lock().unwrap().clone(),
            vec![PYDefinition::Class(PYClass {
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
}
