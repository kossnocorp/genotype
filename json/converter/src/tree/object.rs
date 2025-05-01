use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTObject> for GtjObject {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTObject {
        let name = context.claim_name(self.name.clone(), "Object");

        let properties =
            context.enter_name_context(GTNamingContextName::Identifier(name.clone()), |context| {
                self.properties
                    .iter()
                    .map(|property| property.to_tree_with_context(context))
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

impl GtjTreeConvert<GTDescriptor> for GtjObject {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTDescriptor {
        GTDescriptor::Object(self.to_tree_with_context(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_named() {
        let object = GtjObject {
            r#type: GtjObjectTypeObject,
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
            object.to_tree_with_context(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_unnamed() {
        let object = GtjObject {
            r#type: GtjObjectTypeObject,
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
            object.to_tree_with_context(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_properties() {
        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            name: None,
            doc: None,
            properties: vec![GtjProperty {
                r#type: GtjPropertyTypeProperty,
                name: "null".into(),
                doc: None,
                descriptor: GtjAny::GtjNull(GtjNull {
                    r#type: GtjNullTypeNull,
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
            object.to_tree_with_context(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_naming_unnamed() {
        let mut context = GtjTreeConvertContext::new();

        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            properties: vec![GtjProperty {
                r#type: GtjPropertyTypeProperty,
                name: "hello".into(),
                doc: None,
                descriptor: GtjAny::GtjObject(GtjObject {
                    r#type: GtjObjectTypeObject,
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
            object.to_tree_with_context(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_unnamed_nested() {
        let mut context = GtjTreeConvertContext::new();

        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            properties: vec![GtjProperty {
                r#type: GtjPropertyTypeProperty,
                name: "hello".into(),
                doc: None,
                descriptor: GtjAny::GtjObject(GtjObject {
                    r#type: GtjObjectTypeObject,
                    name: None,
                    doc: None,
                    properties: vec![GtjProperty {
                        r#type: GtjPropertyTypeProperty,
                        name: "world".into(),
                        doc: None,
                        descriptor: GtjAny::GtjObject(GtjObject {
                            r#type: GtjObjectTypeObject,
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
            object.to_tree_with_context(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named() {
        let mut context = GtjTreeConvertContext::new();

        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            properties: vec![GtjProperty {
                r#type: GtjPropertyTypeProperty,
                name: "world".into(),
                doc: None,
                descriptor: GtjAny::GtjObject(GtjObject {
                    r#type: GtjObjectTypeObject,
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
            object.to_tree_with_context(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named_nested() {
        let mut context = GtjTreeConvertContext::new();

        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            properties: vec![GtjProperty {
                r#type: GtjPropertyTypeProperty,
                name: "world".into(),
                doc: None,
                descriptor: GtjAny::GtjObject(GtjObject {
                    r#type: GtjObjectTypeObject,
                    name: Some("Hi".into()),
                    doc: None,
                    properties: vec![GtjProperty {
                        r#type: GtjPropertyTypeProperty,
                        name: "world".into(),
                        doc: None,
                        descriptor: GtjAny::GtjObject(GtjObject {
                            r#type: GtjObjectTypeObject,
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
            object.to_tree_with_context(&mut context),
        );
    }
}
