use genotype_json_tree::*;
use genotype_parser::*;

use crate::{GtjConvert, GtjConvertContext};

impl GtjConvert<GTUnion> for GtjUnion {
    fn convert(&self, context: &mut GtjConvertContext) -> GTUnion {
        let name = context.claim_name(self.name.clone(), "Union");

        let descriptors =
            context.enter_name_context(GTNamingContextName::Identifier(name.clone()), |context| {
                self.descriptors
                    .iter()
                    .map(|descriptor| {
                        context.enter_name_context(
                            GTNamingContextName::Transitive("Member".into()),
                            |context| descriptor.convert(context),
                        )
                    })
                    .collect()
            });

        GTUnion {
            span: GTSpan::default(),
            descriptors,
        }
    }
}

impl GtjConvert<GTDescriptor> for GtjUnion {
    fn convert(&self, context: &mut GtjConvertContext) -> GTDescriptor {
        GTDescriptor::Union(self.convert(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let union = GtjUnion {
            kind: GtjUnionKindUnion,
            descriptors: vec![],
            name: None,
            doc: None,
        };
        assert_eq!(
            GTUnion {
                span: Default::default(),
                descriptors: vec![]
            },
            union.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptors() {
        let union = GtjUnion {
            kind: GtjUnionKindUnion,
            descriptors: vec![GtjAny::GtjNull(GtjNull {
                kind: GtjNullKindNull,
                name: None,
                doc: None,
            })],
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Union(GTUnion {
                span: Default::default(),
                descriptors: vec![GTPrimitive::Null(Default::default()).into()]
            }),
            union.convert(&mut Default::default())
        );
    }

    #[test]
    fn test_convert_naming_unnamed() {
        let mut context = GtjConvertContext::new();

        let union = GtjUnion {
            kind: GtjUnionKindUnion,
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
            GTDescriptor::Union(GTUnion {
                span: Default::default(),
                descriptors: vec![GTObject {
                    span: Default::default(),
                    name: GTObjectName::Named(GTIdentifier(
                        Default::default(),
                        "RootMember".into()
                    )),
                    extensions: vec![],
                    properties: vec![],
                }
                .into()]
            }),
            union.convert(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_unnamed_nested() {
        let mut context = GtjConvertContext::new();

        let union = GtjUnion {
            kind: GtjUnionKindUnion,
            descriptors: vec![GtjAny::GtjUnion(GtjUnion {
                kind: GtjUnionKindUnion,
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
            GTDescriptor::Union(GTUnion {
                span: Default::default(),
                descriptors: vec![GTUnion {
                    span: Default::default(),
                    descriptors: vec![GTObject {
                        span: Default::default(),
                        name: GTObjectName::Named(GTIdentifier(
                            Default::default(),
                            "RootMemberMember".into()
                        )),
                        extensions: vec![],
                        properties: vec![GTProperty {
                            span: Default::default(),
                            descriptor: GTObject {
                                span: Default::default(),
                                name: GTObjectName::Named(GTIdentifier(
                                    Default::default(),
                                    "RootMemberMemberWorld".into()
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
            union.convert(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named() {
        let mut context = GtjConvertContext::new();

        let union = GtjUnion {
            kind: GtjUnionKindUnion,
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
            GTDescriptor::Union(GTUnion {
                span: Default::default(),
                descriptors: vec![GTObject {
                    span: Default::default(),
                    name: GTObjectName::Named(GTIdentifier(Default::default(), "Hello".into())),
                    extensions: vec![],
                    properties: vec![],
                }
                .into()]
            }),
            union.convert(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named_nested() {
        let mut context = GtjConvertContext::new();

        let union = GtjUnion {
            kind: GtjUnionKindUnion,
            descriptors: vec![GtjAny::GtjUnion(GtjUnion {
                kind: GtjUnionKindUnion,
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
            GTDescriptor::Union(GTUnion {
                span: Default::default(),
                descriptors: vec![GTUnion {
                    span: Default::default(),
                    descriptors: vec![GTObject {
                        span: Default::default(),
                        name: GTObjectName::Named(GTIdentifier(
                            Default::default(),
                            "HiMember".into()
                        )),
                        extensions: vec![],
                        properties: vec![GTProperty {
                            span: Default::default(),
                            descriptor: GTObject {
                                span: Default::default(),
                                name: GTObjectName::Named(GTIdentifier(
                                    Default::default(),
                                    "HiMemberWorld".into()
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
            union.convert(&mut context),
        );
    }
}
