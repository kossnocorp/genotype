use genotype_json_tree::*;
use genotype_parser::*;

use crate::{GtjConvert, GtjConvertContext};

impl GtjConvert<GTObject> for GtjObject {
    fn convert(&self, context: &mut GtjConvertContext) -> GTObject {
        let name = context.claim_name(self.name.clone(), "Object");

        let properties =
            context.enter_name_context(GTNamingContextName::Identifier(name.clone()), |context| {
                self.properties
                    .iter()
                    .map(|property| property.convert(context))
                    .collect()
            });

        GTObject {
            span: Default::default(),
            name: GTObjectName::Named(GTIdentifier(Default::default(), name)),
            properties,
            extensions: vec![],
        }
    }
}

impl GtjConvert<GTDescriptor> for GtjObject {
    fn convert(&self, context: &mut GtjConvertContext) -> GTDescriptor {
        GTDescriptor::Object(self.convert(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_named() {
        let object = GtjObject {
            kind: GtjObjectKindObject,
            name: Some("Hello".into()),
            doc: None,
            properties: vec![],
        };
        assert_eq!(
            GTObject {
                span: Default::default(),
                name: GTObjectName::Named(GTIdentifier(Default::default(), "Hello".into())),
                extensions: vec![],
                properties: vec![],
            },
            object.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_unnamed() {
        let object = GtjObject {
            kind: GtjObjectKindObject,
            name: None,
            doc: None,
            properties: vec![],
        };
        assert_eq!(
            GTObject {
                span: Default::default(),
                name: GTObjectName::Named(GTIdentifier(Default::default(), "Root".into())),
                extensions: vec![],
                properties: vec![],
            },
            object.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_properties() {
        let object = GtjObject {
            kind: GtjObjectKindObject,
            name: None,
            doc: None,
            properties: vec![GtjProperty {
                kind: GtjPropertyKindProperty,
                name: "null".into(),
                doc: None,
                descriptor: GtjAny::GtjNull(GtjNull {
                    kind: GtjNullKindNull,
                    name: None,
                    doc: None,
                }),
                required: true,
            }],
        };
        assert_eq!(
            GTObject {
                span: Default::default(),
                name: GTObjectName::Named(GTIdentifier(Default::default(), "Root".into())),
                extensions: vec![],
                properties: vec![GTProperty {
                    span: Default::default(),
                    name: GTKey::new(Default::default(), "null".into()),
                    doc: None,
                    attributes: vec![],
                    descriptor: GTPrimitive::Null(Default::default()).into(),
                    required: true,
                }],
            },
            object.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_naming_unnamed() {
        let mut context = GtjConvertContext::new();

        let object = GtjObject {
            kind: GtjObjectKindObject,
            properties: vec![GtjProperty {
                kind: GtjPropertyKindProperty,
                name: "hello".into(),
                doc: None,
                descriptor: GtjAny::GtjObject(GtjObject {
                    kind: GtjObjectKindObject,
                    name: None,
                    doc: None,
                    properties: vec![],
                }),
                required: false,
            }],
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Object(GTObject {
                span: Default::default(),
                name: GTObjectName::Named(GTIdentifier(Default::default(), "Root".into())),
                extensions: vec![],
                properties: vec![GTProperty {
                    span: Default::default(),
                    descriptor: GTObject {
                        span: Default::default(),
                        name: GTObjectName::Named(GTIdentifier(
                            Default::default(),
                            "RootHello".into()
                        )),
                        extensions: vec![],
                        properties: vec![],
                    }
                    .into(),
                    attributes: Default::default(),
                    required: false,
                    name: GTKey(Default::default(), "hello".into()),
                    doc: None,
                }],
            }),
            object.convert(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_unnamed_nested() {
        let mut context = GtjConvertContext::new();

        let object = GtjObject {
            kind: GtjObjectKindObject,
            properties: vec![GtjProperty {
                kind: GtjPropertyKindProperty,
                name: "hello".into(),
                doc: None,
                descriptor: GtjAny::GtjObject(GtjObject {
                    kind: GtjObjectKindObject,
                    name: None,
                    doc: None,
                    properties: vec![GtjProperty {
                        kind: GtjPropertyKindProperty,
                        name: "world".into(),
                        doc: None,
                        descriptor: GtjAny::GtjObject(GtjObject {
                            kind: GtjObjectKindObject,
                            name: None,
                            doc: None,
                            properties: vec![],
                        }),
                        required: false,
                    }],
                }),
                required: false,
            }],
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Object(GTObject {
                span: Default::default(),
                name: GTObjectName::Named(GTIdentifier(Default::default(), "Root".into())),
                extensions: vec![],
                properties: vec![GTProperty {
                    span: Default::default(),
                    descriptor: GTObject {
                        span: Default::default(),
                        name: GTObjectName::Named(GTIdentifier(
                            Default::default(),
                            "RootHello".into()
                        )),
                        extensions: vec![],
                        properties: vec![GTProperty {
                            span: Default::default(),
                            descriptor: GTObject {
                                span: Default::default(),
                                name: GTObjectName::Named(GTIdentifier(
                                    Default::default(),
                                    "RootHelloWorld".into()
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
                    .into(),
                    attributes: Default::default(),
                    required: false,
                    name: GTKey(Default::default(), "hello".into()),
                    doc: None,
                }],
            }),
            object.convert(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named() {
        let mut context = GtjConvertContext::new();

        let object = GtjObject {
            kind: GtjObjectKindObject,
            properties: vec![GtjProperty {
                kind: GtjPropertyKindProperty,
                name: "world".into(),
                doc: None,
                descriptor: GtjAny::GtjObject(GtjObject {
                    kind: GtjObjectKindObject,
                    name: None,
                    doc: None,
                    properties: vec![],
                }),
                required: false,
            }],
            name: Some("Hello".into()),
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Object(GTObject {
                span: Default::default(),
                name: GTObjectName::Named(GTIdentifier(Default::default(), "Hello".into())),
                extensions: vec![],
                properties: vec![GTProperty {
                    span: Default::default(),
                    descriptor: GTObject {
                        span: Default::default(),
                        name: GTObjectName::Named(GTIdentifier(
                            Default::default(),
                            "HelloWorld".into()
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
            }),
            object.convert(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named_nested() {
        let mut context = GtjConvertContext::new();

        let object = GtjObject {
            kind: GtjObjectKindObject,
            properties: vec![GtjProperty {
                kind: GtjPropertyKindProperty,
                name: "world".into(),
                doc: None,
                descriptor: GtjAny::GtjObject(GtjObject {
                    kind: GtjObjectKindObject,
                    name: Some("Hi".into()),
                    doc: None,
                    properties: vec![GtjProperty {
                        kind: GtjPropertyKindProperty,
                        name: "world".into(),
                        doc: None,
                        descriptor: GtjAny::GtjObject(GtjObject {
                            kind: GtjObjectKindObject,
                            name: None,
                            doc: None,
                            properties: vec![],
                        }),
                        required: false,
                    }],
                }),
                required: false,
            }],
            name: Some("Hello".into()),
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Object(GTObject {
                span: Default::default(),
                name: GTObjectName::Named(GTIdentifier(Default::default(), "Hello".into())),
                extensions: vec![],
                properties: vec![GTProperty {
                    span: Default::default(),
                    descriptor: GTObject {
                        span: Default::default(),
                        name: GTObjectName::Named(GTIdentifier(Default::default(), "Hi".into())),
                        extensions: vec![],
                        properties: vec![GTProperty {
                            span: Default::default(),
                            descriptor: GTObject {
                                span: Default::default(),
                                name: GTObjectName::Named(GTIdentifier(
                                    Default::default(),
                                    "HiWorld".into()
                                )),
                                extensions: vec![],
                                properties: vec![],
                            }
                            .into(),
                            attributes: Default::default(),
                            required: false,
                            name: GTKey(Default::default(), "world".into()),
                            doc: None,
                        },],
                    }
                    .into(),
                    attributes: Default::default(),
                    required: false,
                    name: GTKey(Default::default(), "world".into()),
                    doc: None,
                }],
            }),
            object.convert(&mut context),
        );
    }
}
