use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTTuple> for GtjTuple {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTTuple {
        let name = context.claim_name(self.name.clone(), "Tuple");

        let descriptors =
            context.enter_name_context(GTNamingContextName::Identifier(name.clone()), |context| {
                self.descriptors
                    .iter()
                    .map(|descriptor| {
                        context.enter_name_context(
                            GTNamingContextName::Transitive("Element".into()),
                            |context| descriptor.to_tree_with_context(context),
                        )
                    })
                    .collect()
            });

        GTTuple {
            span: GTSpan::default(),
            descriptors,
        }
    }
}

impl GtjTreeConvert<GTDescriptor> for GtjTuple {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTDescriptor {
        GTDescriptor::Tuple(self.to_tree_with_context(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            descriptors: vec![],
            name: None,
            doc: None,
        };
        assert_eq!(
            GTTuple {
                span: Default::default(),
                descriptors: vec![]
            },
            tuple.to_tree_with_context(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptors() {
        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            descriptors: vec![GtjAny::GtjNull(GtjNull {
                r#type: GtjNullTypeNull,
                name: None,
                doc: None,
            })],
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Tuple(GTTuple {
                span: Default::default(),
                descriptors: vec![GTPrimitive::Null(Default::default()).into()]
            }),
            tuple.to_tree_with_context(&mut Default::default())
        );
    }

    #[test]
    fn test_convert_naming_unnamed() {
        let mut context = GtjTreeConvertContext::new();

        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            descriptors: vec![GtjAny::GtjObject(GtjObject {
                r#type: GtjObjectTypeObject,
                name: None,
                doc: None,
                properties: vec![],
            })],
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Tuple(GTTuple {
                span: Default::default(),
                descriptors: vec![GTObject {
                    span: Default::default(),
                    name: GTObjectName::Named(GTIdentifier(
                        Default::default(),
                        "RootElement".into()
                    )),
                    extensions: vec![],
                    properties: vec![],
                }
                .into()]
            }),
            tuple.to_tree_with_context(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_unnamed_nested() {
        let mut context = GtjTreeConvertContext::new();

        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            descriptors: vec![GtjAny::GtjTuple(GtjTuple {
                r#type: GtjTupleTypeTuple,
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
        assert_eq!(
            GTDescriptor::Tuple(GTTuple {
                span: Default::default(),
                descriptors: vec![GTTuple {
                    span: Default::default(),
                    descriptors: vec![GTObject {
                        span: Default::default(),
                        name: GTObjectName::Named(GTIdentifier(
                            Default::default(),
                            "RootElementElement".into()
                        )),
                        extensions: vec![],
                        properties: vec![GTProperty {
                            span: Default::default(),
                            descriptor: GTObject {
                                span: Default::default(),
                                name: GTObjectName::Named(GTIdentifier(
                                    Default::default(),
                                    "RootElementElementWorld".into()
                                )),
                                extensions: vec![],
                                properties: vec![],
                            }
                            .into(),
                            attributes: Default::default(),
                            required: false,
                            name: GTKey(Default::default(), "world".into()),
                            doc: None,
                        }],
                    }
                    .into()]
                }
                .into()]
            }),
            tuple.to_tree_with_context(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named() {
        let mut context = GtjTreeConvertContext::new();

        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            descriptors: vec![GtjAny::GtjObject(GtjObject {
                r#type: GtjObjectTypeObject,
                name: Some("Hello".into()),
                doc: None,
                properties: vec![],
            })],
            name: Some("World".into()),
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Tuple(GTTuple {
                span: Default::default(),
                descriptors: vec![GTObject {
                    span: Default::default(),
                    name: GTObjectName::Named(GTIdentifier(Default::default(), "Hello".into())),
                    extensions: vec![],
                    properties: vec![],
                }
                .into()]
            }),
            tuple.to_tree_with_context(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named_nested() {
        let mut context = GtjTreeConvertContext::new();

        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            descriptors: vec![GtjAny::GtjTuple(GtjTuple {
                r#type: GtjTupleTypeTuple,
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
        assert_eq!(
            GTDescriptor::Tuple(GTTuple {
                span: Default::default(),
                descriptors: vec![GTTuple {
                    span: Default::default(),
                    descriptors: vec![GTObject {
                        span: Default::default(),
                        name: GTObjectName::Named(GTIdentifier(
                            Default::default(),
                            "HiElement".into()
                        )),
                        extensions: vec![],
                        properties: vec![GTProperty {
                            span: Default::default(),
                            descriptor: GTObject {
                                span: Default::default(),
                                name: GTObjectName::Named(GTIdentifier(
                                    Default::default(),
                                    "HiElementWorld".into()
                                )),
                                extensions: vec![],
                                properties: vec![],
                            }
                            .into(),
                            attributes: Default::default(),
                            required: false,
                            name: GTKey(Default::default(), "world".into()),
                            doc: None,
                        }],
                    }
                    .into()]
                }
                .into()]
            }),
            tuple.to_tree_with_context(&mut context),
        );
    }
}
