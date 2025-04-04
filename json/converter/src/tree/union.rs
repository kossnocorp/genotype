use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTUnion> for GtjUnion {
    fn to_tree(&self, context: &mut GtjTreeConvertContext) -> GTUnion {
        let name = context.claim_name(self.name.clone(), "Union");

        let descriptors =
            context.enter_name_context(GTNamingContextName::Identifier(name.clone()), |context| {
                self.descriptors
                    .iter()
                    .map(|descriptor| {
                        context.enter_name_context(
                            GTNamingContextName::Transitive("Member".into()),
                            |context| descriptor.to_tree(context),
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

impl GtjTreeConvert<GTDescriptor> for GtjUnion {
    fn to_tree(&self, context: &mut GtjTreeConvertContext) -> GTDescriptor {
        GTDescriptor::Union(self.to_tree(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
            descriptors: vec![],
            name: None,
            doc: None,
        };
        assert_eq!(
            GTUnion {
                span: Default::default(),
                descriptors: vec![]
            },
            union.to_tree(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptors() {
        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
            descriptors: vec![GtjAny::GtjNull(GtjNull {
                r#type: GtjNullTypeNull,
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
            union.to_tree(&mut Default::default())
        );
    }

    #[test]
    fn test_convert_naming_unnamed() {
        let mut context = GtjTreeConvertContext::new();

        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
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
            union.to_tree(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_unnamed_nested() {
        let mut context = GtjTreeConvertContext::new();

        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
            descriptors: vec![GtjAny::GtjUnion(GtjUnion {
                r#type: GtjUnionTypeUnion,
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
            union.to_tree(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named() {
        let mut context = GtjTreeConvertContext::new();

        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
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
            union.to_tree(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named_nested() {
        let mut context = GtjTreeConvertContext::new();

        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
            descriptors: vec![GtjAny::GtjUnion(GtjUnion {
                r#type: GtjUnionTypeUnion,
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
            union.to_tree(&mut context),
        );
    }
}
