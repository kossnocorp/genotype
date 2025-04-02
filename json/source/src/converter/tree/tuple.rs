use genotype_json_schema::*;
use genotype_parser::*;

use crate::{GtjConvert, GtjConvertContext};

impl GtjConvert<GTTuple> for GtjTuple {
    fn convert(&self, context: &mut GtjConvertContext) -> GTTuple {
        let name = context.claim_name(self.name.clone(), "Tuple");

        let descriptors =
            context.enter_name_context(GTNamingContextName::Identifier(name.clone()), |context| {
                self.descriptors
                    .iter()
                    .map(|descriptor| {
                        context.enter_name_context(
                            GTNamingContextName::Transitive("Element".into()),
                            |context| descriptor.convert(context),
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

impl GtjConvert<GTDescriptor> for GtjTuple {
    fn convert(&self, context: &mut GtjConvertContext) -> GTDescriptor {
        GTDescriptor::Tuple(self.convert(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let tuple = GtjTuple {
            kind: GtjTupleKindTuple,
            descriptors: vec![],
            name: None,
            doc: None,
        };
        assert_eq!(
            GTTuple {
                span: Default::default(),
                descriptors: vec![]
            },
            tuple.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptors() {
        let tuple = GtjTuple {
            kind: GtjTupleKindTuple,
            descriptors: vec![GtjAny::GtjNull(GtjNull {
                kind: GtjNullKindNull,
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
            tuple.convert(&mut Default::default())
        );
    }

    #[test]
    fn test_convert_naming_unnamed() {
        let mut context = GtjConvertContext::new();

        let tuple = GtjTuple {
            kind: GtjTupleKindTuple,
            descriptors: vec![GtjAny::GtjObject(GtjObject {
                kind: GtjObjectKindObject,
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
            tuple.convert(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_unnamed_nested() {
        let mut context = GtjConvertContext::new();

        let tuple = GtjTuple {
            kind: GtjTupleKindTuple,
            descriptors: vec![GtjAny::GtjTuple(GtjTuple {
                kind: GtjTupleKindTuple,
                descriptors: vec![GtjAny::GtjObject(GtjObject {
                    kind: GtjObjectKindObject,
                    name: None,
                    doc: None,
                    properties: vec![GtjProperty {
                        kind: GtjPropertyKindProperty,
                        name: "world".into(),
                        doc: None,
                        descriptor: GtjAny::GtjObject(GtjObject {
                            name: None,
                            doc: None,
                            kind: GtjObjectKindObject,
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
            tuple.convert(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named() {
        let mut context = GtjConvertContext::new();

        let tuple = GtjTuple {
            kind: GtjTupleKindTuple,
            descriptors: vec![GtjAny::GtjObject(GtjObject {
                kind: GtjObjectKindObject,
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
            tuple.convert(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named_nested() {
        let mut context = GtjConvertContext::new();

        let tuple = GtjTuple {
            kind: GtjTupleKindTuple,
            descriptors: vec![GtjAny::GtjTuple(GtjTuple {
                kind: GtjTupleKindTuple,
                descriptors: vec![GtjAny::GtjObject(GtjObject {
                    kind: GtjObjectKindObject,
                    name: None,
                    doc: None,
                    properties: vec![GtjProperty {
                        kind: GtjPropertyKindProperty,
                        name: "world".into(),
                        doc: None,
                        descriptor: GtjAny::GtjObject(GtjObject {
                            name: None,
                            doc: None,
                            kind: GtjObjectKindObject,
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
            tuple.convert(&mut context),
        );
    }
}
