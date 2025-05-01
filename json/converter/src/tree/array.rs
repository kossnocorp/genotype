use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTArray> for GtjArray {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTArray {
        let name = context.claim_name(self.name.clone(), "Array");

        let descriptor =
            context.enter_name_context(GTNamingContextName::Identifier(name.clone()), |context| {
                context.enter_name_context(
                    GTNamingContextName::Transitive("Element".into()),
                    |context| self.descriptor.to_tree_with_context(context),
                )
            });

        GTArray {
            span: GTSpan::default(),
            descriptor,
        }
    }
}

impl GtjTreeConvert<GTDescriptor> for GtjArray {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTDescriptor {
        GTDescriptor::Array(Box::new(self.to_tree_with_context(context)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let array = GtjArray {
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjNull(GtjNull {
                r#type: GtjNullTypeNull,
                name: None,
                doc: None,
            }),
            name: None,
            doc: None,
        };
        assert_eq!(
            GTArray {
                span: Default::default(),
                descriptor: GTPrimitive::Null(Default::default()).into()
            },
            array.to_tree_with_context(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptor() {
        let array = GtjArray {
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjNull(GtjNull {
                r#type: GtjNullTypeNull,
                name: None,
                doc: None,
            }),
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: Default::default(),
                descriptor: GTPrimitive::Null(Default::default()).into()
            })),
            array.to_tree_with_context(&mut Default::default())
        );
    }

    #[test]
    fn test_convert_naming_unnamed() {
        let mut context = GtjTreeConvertContext::new();

        let array = GtjArray {
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjObject(GtjObject {
                r#type: GtjObjectTypeObject,
                name: None,
                doc: None,
                properties: vec![],
            }),
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: Default::default(),
                descriptor: GTObject {
                    span: Default::default(),
                    name: GTObjectName::Named(GTIdentifier(
                        Default::default(),
                        "RootElement".into()
                    )),
                    extensions: vec![],
                    properties: vec![],
                }
                .into()
            })),
            array.to_tree_with_context(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_unnamed_nested() {
        let mut context = GtjTreeConvertContext::new();

        let array = GtjArray {
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjArray(Box::new(GtjArray {
                r#type: GtjArrayTypeArray,
                descriptor: GtjAny::GtjObject(GtjObject {
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
                }),
                name: None,
                doc: None,
            })),
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: Default::default(),
                descriptor: GTArray {
                    span: Default::default(),
                    descriptor: GTObject {
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
                    .into()
                }
                .into()
            })),
            array.to_tree_with_context(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named() {
        let mut context = GtjTreeConvertContext::new();

        let array = GtjArray {
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjObject(GtjObject {
                r#type: GtjObjectTypeObject,
                name: None,
                doc: None,
                properties: vec![],
            }),
            name: Some("Hello".into()),
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: Default::default(),
                descriptor: GTObject {
                    span: Default::default(),
                    name: GTObjectName::Named(GTIdentifier(
                        Default::default(),
                        "HelloElement".into()
                    )),
                    extensions: vec![],
                    properties: vec![],
                }
                .into()
            })),
            array.to_tree_with_context(&mut context),
        );
    }

    #[test]
    fn test_convert_naming_named_nested() {
        let mut context = GtjTreeConvertContext::new();

        let array = GtjArray {
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjArray(Box::new(GtjArray {
                r#type: GtjArrayTypeArray,
                descriptor: GtjAny::GtjObject(GtjObject {
                    r#type: GtjObjectTypeObject,
                    name: Some("Hey".into()),
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
                }),
                name: Some("Hi".into()),
                doc: None,
            })),
            name: Some("Hello".into()),
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: Default::default(),
                descriptor: GTArray {
                    span: Default::default(),
                    descriptor: GTObject {
                        span: Default::default(),
                        name: GTObjectName::Named(GTIdentifier(Default::default(), "Hey".into())),
                        extensions: vec![],
                        properties: vec![GTProperty {
                            span: Default::default(),
                            descriptor: GTObject {
                                span: Default::default(),
                                name: GTObjectName::Named(GTIdentifier(
                                    Default::default(),
                                    "HeyWorld".into()
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
                    .into()
                }
                .into()
            })),
            array.to_tree_with_context(&mut context),
        );
    }
}
